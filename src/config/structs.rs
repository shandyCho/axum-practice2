pub mod config_structurs {
    use std::sync::Arc;

    use axum::{body::Body, extract::Request};
    use rdkafka::producer::FutureProducer;
    use tower_http::trace::OnRequest;
    use tracing::Span;    
    use crate::send_view_log::send_view_log::send_view_log;

    #[derive(Clone, Debug)]
    pub struct CustomDefaultOnRequest;

    impl OnRequest<Body> for CustomDefaultOnRequest {
        fn on_request(&mut self, request: &Request<Body>, _span: &Span) {
            tracing::info!("started {} {}", request.method(), request.uri().path());

            // request.uri() 서브스트링 해서 dashboard 라는 단어가 들어가 있을 경우
            // send_view_log 메서드를 실행하도록 하기
            
            // let kafka_state = KafkaState.getter();
            // send_view_log(kafka_state, "music".to_string(), 1);
        }
    }

    #[derive(Clone)]
    pub struct KafkaState {
        pub(crate) producer: Arc<FutureProducer>,
    }    

    pub trait Getter {
        fn getter(&self) -> Arc<FutureProducer>;
    }

    impl Getter for KafkaState {
        fn getter(&self) -> Arc<FutureProducer> {
            self.producer.clone()
        }
    }
}