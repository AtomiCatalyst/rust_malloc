pub mod node {
    #[derive(Debug)]
    pub struct Node_t<'a> {
        pub size: usize,
        pub is_free: bool,
        pub fwd: Option<&'a Node_t<'a>>,
        pub bwd: Option<&'a Node_t<'a>>
    }
}