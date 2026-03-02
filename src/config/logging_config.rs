pub mod config1 {
    use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
    use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
    use tracing::{Level};

    pub fn logging_setup() -> TraceLayer<
        SharedClassifier<ServerErrorsAsFailures>,
        DefaultMakeSpan,
        DefaultOnRequest,
        DefaultOnResponse,
        > {
        let trace_layer = TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_request(DefaultOnRequest::new()
                    .level(Level::INFO))
                .on_response(DefaultOnResponse::new()
                    .level(Level::INFO));        
        trace_layer
    }   
}   

pub mod config2 {
    use std::time::Duration;

    use axum::middleware::Next;
    use axum::response::Response;
    use axum::{body::{to_bytes, Body}, extract::Request};
    use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, OnRequest, OnResponse, TraceLayer};
    use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
    use tracing::{Level, Span};

    #[derive(Clone, Debug)]
    pub struct CustomDefaultOnRequest;

    impl OnRequest<Body> for CustomDefaultOnRequest {
        fn on_request(&mut self, request: &Request<Body>, _span: &Span) {
            let request = request.clone();
            tracing::info!("started {} {} ", request.method(), request.uri().path()) 
        }
    }

    #[derive(Clone, Debug)]
    pub struct CustomDefaultOnResponse;

    impl OnResponse<Body> for CustomDefaultOnResponse {
        fn on_response(self, response: &Response<Body>, latency: Duration, _span: &Span) {
            let res = response.extensions().get::<String>();
            match res {
                None => {
                    tracing::info!("response generated in {:?}", latency)
                },
                Some(dashboard_string) => tracing::info!("response status: {} headers: {:?} body {}", response.status(), response.headers(), *dashboard_string),
            }
        }
    }

    pub fn logging_setup2() -> TraceLayer<
        SharedClassifier<ServerErrorsAsFailures>,
        DefaultMakeSpan,
        (),
        // CustomDefaultOnRequest,
        CustomDefaultOnResponse,
        > {
        let trace_layer = TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(())
                // .on_request(CustomDefaultOnRequest)
                .on_response(CustomDefaultOnResponse);
        
        trace_layer
    }

    pub async fn logging_request(req: Request, next: Next) -> Response {
        // Request를 받아서 할 작업 작성
        // 이 곳에서 Request의 바디를 로깅하고 새 Request를 만들어서 넘겨준다
        // 헤더 정보 클론
        let headers = req.headers().clone();
        // HTTP Method 정보 클론
        let method = req.method().clone();
        // URI 정보 클론
        let uri = req.uri().clone();
        // Body 정보 바이트로 변환
        let body = to_bytes(req.into_body(), usize::MAX).await.unwrap();
        // 바이트로 변환된 Body를 문자열로 변환 (UTF-8로 가정)
        let body_string = String::from_utf8_lossy(&body);
        // 로그 출력
        tracing::info!("request method: {} uri: {} body: {}", method, uri, &body_string);
        // 새로운 Request 생성 (원래의 Request에서 Bytes로 변환된 Body를 다시 Body로 변환하여 사용)
        let new_body = Body::from(body);
        let mut request = Request::builder()
            .method(method)
            .uri(uri)
            .body(new_body)
            .unwrap();
        headers.iter().for_each(|(key, value)| {
            // 새 Request에 헤더 정보 추가
            request.headers_mut().insert(key, value.clone());
        });
        
        let response = next.run(request).await;
        // Response를 받아서 할 작업 작성
        response
    }
}