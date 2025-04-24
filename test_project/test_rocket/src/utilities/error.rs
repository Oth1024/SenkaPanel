use std::error::Error;

pub struct DemoError;

impl Error for DemoError {
    fn cause(&self) -> Option<&dyn Error> {
        
    }

    fn description(&self) -> &str {
        
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        
    }
}

impl FromResidual for DemoError {
    
}