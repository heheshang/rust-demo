// trait object

fn main() {
    struct R<'a>(&'a i32);
    // 将 'b 生命周期延长至 'static 生命周期
    unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
        std::mem::transmute::<R<'b>, R<'static>>(r)
    }
    // 将 'static 生命周期缩短至 'c 生命周期
    unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
        std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
    }
}
