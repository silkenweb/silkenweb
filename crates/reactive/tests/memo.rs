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
#[should_panic]
fn reuse_key_within_frame() {
    let memo = MemoCache::new();
    let frame = memo.frame();

    frame.cache(0, || 1);
    frame.cache(0, || 2);
}
