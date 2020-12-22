use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TVErr { // Tree Validation Error
  msg: String,
  from: Option<Box<TVErr>>
}

impl fmt::Display for TVErr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &self.from {
      Some(b) => { write!(f, "{}", *b)?; },
      None => ()
    }
    write!(f, "{}", self.msg)
  }
}

impl Error for TVErr {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match &self.from {
      Some(b) => Some(b),
      None => None
    }
  }
}

impl TVErr {
  pub fn origin<S: Into<String>>(msg: S) -> TVErr {
    TVErr {
      msg: msg.into(),
      from: None
    }
  }

  pub fn wrap<S: Into<String>>(from: TVErr, msg: S) -> TVErr {
    TVErr {
      msg: msg.into(),
      from: Some(Box::new(from))
    }
  }
}