//! Performance benchmarks for authentication operations
//!
//! Task 4.7: Verify performance targets:
//! - Password hashing: < 500ms (Argon2id parameters)
//! - Session validation: < 5ms overhead
//! - Token generation: < 1ms

use altair_auth::local::{generate_token, hash_password, verify_password};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

/// Benchmark password hashing performance
///
/// Target: < 500ms for Argon2id with params (time=3, memory=65536, parallelism=4)
fn bench_password_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("password_hashing");

    // Set sample size to reduce total benchmark time (password hashing is slow)
    group.sample_size(10);

    let rt = tokio::runtime::Runtime::new().unwrap();

    group.bench_function("hash_password", |b| {
        b.iter(|| {
            let password = black_box("test_password_123");
            rt.block_on(hash_password(password))
                .expect("Hashing should succeed")
        });
    });

    group.finish();
}

/// Benchmark password verification performance
///
/// Target: < 500ms (same as hashing, constant-time comparison)
fn bench_password_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("password_verification");

    // Set sample size to reduce total benchmark time
    group.sample_size(10);

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Pre-generate hash for verification tests
    let password = "test_password_123";
    let hash = rt
        .block_on(hash_password(password))
        .expect("Hashing should succeed");

    group.bench_function("verify_correct_password", |b| {
        b.iter(|| {
            rt.block_on(verify_password(black_box(password), black_box(&hash)))
                .expect("Verification should succeed")
        });
    });

    group.bench_function("verify_wrong_password", |b| {
        b.iter(|| {
            let wrong_password = black_box("wrong_password_456");
            rt.block_on(verify_password(wrong_password, black_box(&hash)))
                .expect("Verification should succeed (returns false)")
        });
    });

    group.finish();
}

/// Benchmark token generation performance
///
/// Target: < 1ms for 256-bit random token generation and hex encoding
fn bench_token_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("token_generation");

    group.bench_function("generate_token", |b| {
        b.iter(generate_token);
    });

    // Verify token length (64 hex chars = 32 bytes * 2)
    let token = generate_token();
    assert_eq!(token.len(), 64, "Token should be 64 hex characters");

    group.finish();
}

/// Benchmark token comparison operations
///
/// Verify that token operations are fast (should be microseconds)
fn bench_token_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("token_operations");

    let token1 = generate_token();
    let token2 = generate_token();
    let token1_copy = token1.clone();

    group.bench_function("token_equality_same", |b| {
        b.iter(|| black_box(&token1) == black_box(&token1_copy));
    });

    group.bench_function("token_equality_different", |b| {
        b.iter(|| black_box(&token1) == black_box(&token2));
    });

    group.finish();
}

/// Combined benchmark group to test realistic auth flow performance
fn bench_auth_flow_combined(c: &mut Criterion) {
    let mut group = c.benchmark_group("combined_auth_flow");

    // Reduce sample size for combined flow (includes slow password operations)
    group.sample_size(10);

    let rt = tokio::runtime::Runtime::new().unwrap();

    group.bench_function("register_flow", |b| {
        b.iter(|| {
            // Simulate registration: hash password + generate token
            let password = black_box("user_password_123");
            let hash = rt
                .block_on(hash_password(password))
                .expect("Hashing should succeed");
            let token = generate_token();

            // Return to prevent compiler optimization
            (hash, token)
        });
    });

    // Pre-generate hash for login flow
    let password = "user_password_123";
    let hash = rt
        .block_on(hash_password(password))
        .expect("Hashing should succeed");

    group.bench_function("login_flow", |b| {
        b.iter(|| {
            // Simulate login: verify password + generate token
            let valid = rt
                .block_on(verify_password(black_box(password), black_box(&hash)))
                .expect("Verification should succeed");

            if valid {
                let token = generate_token();
                Some(token)
            } else {
                None
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_password_hashing,
    bench_password_verification,
    bench_token_generation,
    bench_token_operations,
    bench_auth_flow_combined
);

criterion_main!(benches);
