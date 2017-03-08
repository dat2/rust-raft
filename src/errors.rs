error_chain! {

  types {
    Error, ErrorKind, ResultExt, Result;
  }

  foreign_links {
    Io(::std::io::Error);
    AddrParse(::std::net::AddrParseError);
    Log(::log::SetLoggerError);
  }

}
