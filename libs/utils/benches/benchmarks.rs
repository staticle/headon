use criterion::{Criterion, criterion_group, criterion_main};
use utils::id;

pub fn bench_id_stringify(c: &mut Criterion) {
    // Can only use public methods.
    let ttid = id::TenantTimelineId::generate();

    c.bench_function("id.to_string", |b| {
        b.iter(|| {
            // FIXME measurement overhead?
            //for _ in 0..1000 {
            //    ttid.tenant_id.to_string();
            //}
            ttid.tenant_id.to_string();
        })
    });
}

criterion_group!(benches, bench_id_stringify);
criterion_main!(benches);
