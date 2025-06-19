use criterion::{Criterion, criterion_group, criterion_main};
use graphics::renderer::draw_sprite_block;

const SCREEN_WIDTH: usize = 128;
const SCREEN_HEIGHT: usize = 128;

fn bench_draw_sprite(c: &mut Criterion) {
    let mut group = c.benchmark_group("draw_sprite_block");

    let mut vram = vec![0u8; SCREEN_WIDTH * SCREEN_HEIGHT / 2];
    let sprite = [[0x12, 0x34, 0x56, 0x78]; 8];

    group.bench_function("scale_2x", |b| {
        b.iter(|| {
            draw_sprite_block(
                &mut vram,
                sprite,
                32, 32,
                2.0, 2.0,
                false, false,
            )
        });
    });

    group.finish();
}

criterion_group!(benches, bench_draw_sprite);
criterion_main!(benches);
