use my_retreat_nest::run;

#[cfg(feature = "with-jemalloc")]
use jemallocator::Jemalloc;

#[cfg(feature = "with-jemalloc")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    run().await;
}
