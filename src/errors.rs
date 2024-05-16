
pub mod Error {
    #[derive(Debug)]
    pub enum ErrorType {
        ERR_BAD_ARGUMENTS,
        ERR_SYSCALL_FAILED,
        ERR_UNINITIALIZED,
        ERR_OUT_OF_MEMORY
    }
}