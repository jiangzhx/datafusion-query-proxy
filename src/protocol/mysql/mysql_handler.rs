use datafusion::arrow::datatypes::DataType;
use msql_srv::QueryResultWriter;
use std::{io, net};

pub fn on_query(_sql: &str, _writer: QueryResultWriter<net::TcpStream>) -> io::Result<()> {
	let task = async {};
	tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(task);
	// .unwrap();
	Ok(())
}

pub fn on_prepare(_: &str) -> u32 {
	1
}
pub fn on_execute(
	_: u32,
	_: Vec<msql_srv::ParamValue>,
	results: QueryResultWriter<net::TcpStream>,
) -> Result<(), io::Error> {
	results.completed(0, 0)
}

pub fn arrow_to_mysql(t: &DataType) -> msql_srv::ColumnType {
	match t {
		DataType::Int8 | DataType::UInt8 | DataType::Int16 | DataType::UInt16 => msql_srv::ColumnType::MYSQL_TYPE_TINY,
		DataType::Int32 | DataType::UInt32 => msql_srv::ColumnType::MYSQL_TYPE_SHORT,
		DataType::Int64 | DataType::UInt64 => msql_srv::ColumnType::MYSQL_TYPE_LONG,
		DataType::Float16 | DataType::Float32 => msql_srv::ColumnType::MYSQL_TYPE_FLOAT,
		DataType::Float64 => msql_srv::ColumnType::MYSQL_TYPE_DOUBLE,
		DataType::Utf8 | DataType::LargeUtf8 => msql_srv::ColumnType::MYSQL_TYPE_STRING,
		_ => {
			unimplemented!("don't know how to translate arrow type {:?} to a MySQL type", t);
		}
	}
}
