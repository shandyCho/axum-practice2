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

    use axum::{body::Body, extract::Request, http::Response};
    use tower_http::trace::{DefaultMakeSpan, OnRequest, OnResponse, TraceLayer};
    use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
    use tracing::{Level, Span};

    #[derive(Clone, Debug)]
    pub struct CustomDefaultOnRequest;

    impl OnRequest<Body> for CustomDefaultOnRequest {
        fn on_request(&mut self, request: &Request<Body>, _span: &Span) {
            tracing::info!("started {} {}", request.method(), request.uri().path())
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
        CustomDefaultOnRequest,
        CustomDefaultOnResponse,
        > {
        let trace_layer = TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(CustomDefaultOnRequest)
                .on_response(CustomDefaultOnResponse);
        
        trace_layer
    }
}