use std::mem;

use silkenweb_reactive::memo::MemoCache;

#[test]
fn cross_frame_cache() {
    let memo = MemoCache::new();

    {
        let frame = memo.frame();
        assert_eq!(frame.cache(0, || 1), 1);
    }

    let frame = memo.frame();

    assert_eq!(1, frame.cache(0, || 2), "The old value should be cached");
}

#[test]
fn multi_value_caching() {
    let memo = MemoCache::new();

    {
        let frame = memo.frame();
        assert_eq!(frame.cache(0_u8, || 10), 10);
        assert_eq!(frame.cache(1_u8, || 20), 20);
        assert_eq!(frame.cache(0_u64, || 30), 30);
        assert_eq!(frame.cache(1_u64, || 40), 40);
    }

    let frame = memo.frame();

    assert_eq!(
        frame.cache(0_u8, || 0),
        10,
        "The old value should be cached"
    );
    assert_eq!(
        frame.cache(1_u8, || 0),
        20,
        "The old value should be cached"
    );
    assert_eq!(
        frame.cache(0_u64, || 0),
        30,
        "The old value should be cached"
    );
    assert_eq!(
        frame.cache(1_u64, || 0),
        40,
        "The old value should be cached"
    );
}

#[test]
fn cache_expiry() {
    let memo = MemoCache::new();

    {
        let frame = memo.frame();
        frame.cache(0, || 1);
    }

    {
        let frame = memo.frame();

        assert_eq!(1, frame.cache(0, || 2), "The old value should be cached");
    }

    mem::drop(memo.frame());

    {
        let frame = memo.frame();

        assert_eq!(
            3,
            frame.cache(0, || 3),
            "The old value should expire from the cache"
        );
    }
}

#[test]
#[should_panic]
fn reuse_key_within_frame() {
    let memo = MemoCache::new();
    let frame = memo.frame();

    frame.cache(0, || 1);
    frame.cache(0, || 2);
}
