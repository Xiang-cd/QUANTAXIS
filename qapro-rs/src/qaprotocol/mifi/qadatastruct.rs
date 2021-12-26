extern crate arrow;
use arrow::{
    csv,
    datatypes::{DataType, Field, Schema, SchemaRef},
    record_batch::RecordBatch,
    tensor::Tensor,
    util::pretty::print_batches,
};
use std::{any::Any, collections::BTreeSet, fs::File, sync::Arc};

use datafusion::{
    datasource::TableProvider,
    error::{DataFusionError, Result},
    logical_plan::Expr,
    physical_plan::{
        ColumnStatistics, DisplayFormatType, ExecutionPlan, Partitioning,
        SendableRecordBatchStream, Statistics,
    },
    prelude::{ExecutionContext,CsvReadOptions},
    scalar::ScalarValue,
};
use datafusion::datasource::MemTable;
use datafusion::prelude::DataFrame;

fn read_csv() {
    let file = File::open("testdata.csv").unwrap();
    let builder = csv::ReaderBuilder::new()
        .has_header(true)
        .infer_schema(Some(100));
    let mut csv = builder.build(file).unwrap();
    let batch = csv.next().unwrap().unwrap();
    let data = batch.column(3).data();
    println!("{:#?}", data);

    //print_batches(&[batch]).unwrap();
}

struct ExC{
    ctx : ExecutionContext,
    uri : String
}

impl ExC {
    fn new()-> Self{
        Self{ctx: ExecutionContext::new(), uri: "testdata.csv".to_string()}
    }
    async fn reg_csv(&mut self){
        self.ctx.register_csv("example", self.uri.as_str(),  CsvReadOptions::new()).await.unwrap();
    }

    async fn read_csv(&mut self) -> Arc<dyn DataFrame>{
        let df = self.ctx.read_csv(self.uri.as_str(), CsvReadOptions::new()).await.unwrap();
        df
    }

}


#[actix_rt::test]
async fn t() {
    let mut E =  ExC::new();
    // /E.reg_csv();
    E.ctx.register_csv("example",  "testdata.csv",  CsvReadOptions::new()).await.unwrap();
    let res = E.ctx.sql("SELECT * from example where order_book_id = '000001.XSHE' order by date").await.unwrap();
    let results = res.collect().await.unwrap();


    print_batches(&results).unwrap();
}
