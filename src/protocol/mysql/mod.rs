pub mod mysql_handler;

extern crate futures;
extern crate msql_srv;
extern crate tokio;

use std::error::Error;
use std::net;
use std::net::TcpStream;
use std::{io, thread};

use msql_srv::{Column, MysqlIntermediary, MysqlShim, ParamParser, QueryResultWriter, StatementMetaWriter};
use mysql::OptsBuilder;

pub struct DataFusionBackend {
	columns: Vec<Column>,
	params: Vec<Column>,
}

impl DataFusionBackend {
	pub fn new(scheduler_bind_host: &str, scheduler_bind_port: u16) -> Self {
		DataFusionBackend {
			columns: Vec::new(),
			params: Vec::new(),
		}
	}
	#[allow(dead_code)]
	pub fn test<C>(self, c: C)
	where
		C: FnOnce(&mut mysql::Conn),
	{
		self.test_with_result(c).unwrap()
	}

	#[allow(dead_code)]
	fn test_with_result<C>(self, c: C) -> Result<(), Box<dyn Error + 'static>>
	where
		C: FnOnce(&mut mysql::Conn),
	{
		let listener = net::TcpListener::bind("127.0.0.1:0").unwrap();
		let port = listener.local_addr().unwrap().port();
		thread::spawn(move || {
			let (s, _) = listener.accept().unwrap();
			MysqlIntermediary::run_on_tcp(self, s)
		});

		let opts = OptsBuilder::default().ip_or_hostname(Some("0.0.0.0")).tcp_port(port);
		// .ssl_opts(client_tls);

		let mut db = mysql::Conn::new(opts)?;

		c(&mut db);
		drop(db);
		// jh.join().unwrap().unwrap();

		Ok(())
	}
}

impl MysqlShim<net::TcpStream> for DataFusionBackend {
	type Error = io::Error;

	fn on_prepare(&mut self, query: &str, info: StatementMetaWriter<'_, TcpStream>) -> Result<(), Self::Error> {
		let id = (mysql_handler::on_prepare)(query);
		info.reply(id, &self.params, &self.columns)
	}

	fn on_execute(
		&mut self,
		id: u32,
		params: ParamParser<'_>,
		results: QueryResultWriter<'_, TcpStream>,
	) -> Result<(), Self::Error> {
		(mysql_handler::on_execute)(id, params.into_iter().collect(), results)
	}

	fn on_close(&mut self, _: u32) {}

	fn on_query(&mut self, query: &str, results: QueryResultWriter<'_, TcpStream>) -> Result<(), Self::Error> {
		(mysql_handler::on_query)(query, results)
	}
}
