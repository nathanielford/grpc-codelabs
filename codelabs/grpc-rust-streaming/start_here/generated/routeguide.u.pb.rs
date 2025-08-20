const _: () = ::protobuf::__internal::assert_compatible_gencode_version(
    "4.32.0-release",
);
pub(crate) static mut routeguide__Point_msg_init: ::protobuf::__internal::runtime::MiniTablePtr = ::protobuf::__internal::runtime::MiniTablePtr(
    ::std::ptr::null_mut(),
);
#[allow(non_camel_case_types)]
pub struct Point {
    inner: ::protobuf::__internal::runtime::OwnedMessageInner<Point>,
}
impl ::protobuf::Message for Point {}
impl ::std::default::Default for Point {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for Point {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
    fn parse_dont_enforce_required(
        serialized: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse_dont_enforce_required(serialized)
    }
}
impl ::std::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for Point {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
unsafe impl Sync for Point {}
unsafe impl Send for Point {}
impl ::protobuf::Proxied for Point {
    type View<'msg> = PointView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for Point {}
impl ::protobuf::MutProxied for Point {
    type Mut<'msg> = PointMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PointView<'msg> {
    inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Point>,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for PointView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for PointView<'msg> {
    type Message = Point;
}
impl ::std::fmt::Debug for PointView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for PointView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for PointView<'_> {
    fn default() -> PointView<'static> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(
                ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
            )
        };
        PointView::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
impl<'msg> PointView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Point>,
    ) -> Self {
        Self {
            inner,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    pub fn to_owned(&self) -> Point {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn latitude(self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(0, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn longitude(self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(1, (0i32).into()).try_into().unwrap()
        }
    }
}
unsafe impl Sync for PointView<'_> {}
unsafe impl Send for PointView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for PointView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for PointView<'msg> {}
impl<'msg> ::protobuf::AsView for PointView<'msg> {
    type Proxied = Point;
    fn as_view(&self) -> ::protobuf::View<'msg, Point> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for PointView<'msg> {
    fn into_view<'shorter>(self) -> PointView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<Point> for PointView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Point {
        let mut dst = Point::new();
        let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(
            &self,
            ::protobuf::__internal::Private,
        );
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst_raw,
                src_raw,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst_arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<Point> for PointMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Point {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for Point {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        PointView::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn from_message_mut<'msg>(
        msg: ::protobuf::__internal::runtime::RawMessage,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> PointMut<'msg> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageMutInner::<
                'msg,
                Point,
            >::wrap_raw(msg, arena)
        };
        PointMut::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PointMut<'msg> {
    inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Point>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for PointMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for PointMut<'msg> {
    type Message = Point;
}
impl ::std::fmt::Debug for PointMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for PointMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
#[allow(dead_code)]
impl<'msg> PointMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent<ParentT: ::protobuf::Message>(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Point>,
    ) -> Self {
        Self { inner }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Point> {
        self.inner
    }
    pub fn to_owned(&self) -> Point {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn latitude(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(0, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_latitude(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(0, val.into()) }
    }
    pub fn longitude(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(1, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_longitude(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(1, val.into()) }
    }
}
unsafe impl Sync for PointMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for PointMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for PointMut<'msg> {}
impl<'msg> ::protobuf::AsView for PointMut<'msg> {
    type Proxied = Point;
    fn as_view(&self) -> ::protobuf::View<'_, Point> {
        PointView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for PointMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Point>
    where
        'msg: 'shorter,
    {
        PointView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for PointMut<'msg> {
    type MutProxied = Point;
    fn as_mut(&mut self) -> PointMut<'msg> {
        PointMut { inner: self.inner }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for PointMut<'msg> {
    fn into_mut<'shorter>(self) -> PointMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl Point {
    pub fn new() -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new(),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Point> {
        ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn parse_dont_enforce_required(
        data: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data)
            .map(|_| msg)
    }
    pub fn as_view(&self) -> PointView {
        PointView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner),
        )
    }
    pub fn as_mut(&mut self) -> PointMut {
        let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(
            &mut self.inner,
        );
        PointMut::new(::protobuf::__internal::Private, inner)
    }
    pub fn latitude(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(0, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_latitude(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(0, val.into()) }
    }
    pub fn longitude(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(1, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_longitude(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(1, val.into()) }
    }
}
impl ::std::ops::Drop for Point {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for Point {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for Point {
    type Proxied = Self;
    fn as_view(&self) -> PointView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for Point {
    type MutProxied = Self;
    fn as_mut(&mut self) -> PointMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Point {
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        static ONCE_LOCK: ::std::sync::OnceLock<
            ::protobuf::__internal::runtime::MiniTablePtr,
        > = ::std::sync::OnceLock::new();
        ONCE_LOCK
            .get_or_init(|| unsafe {
                super::routeguide__Point_msg_init.0 = ::protobuf::__internal::runtime::upb_MiniTable_Build(
                    "$(P(P".as_ptr(),
                    5,
                    ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA
                        .with(|a| a.raw()),
                    ::std::ptr::null_mut(),
                );
                let submessages = [];
                let subenums = [];
                assert!(
                    ::protobuf::__internal::runtime::upb_MiniTable_Link(super::routeguide__Point_msg_init
                    .0, submessages.as_ptr() as * const * const
                    ::protobuf::__internal::runtime::upb_MiniTable, submessages.len(),
                    subenums.as_ptr(), subenums.len())
                );
                ::protobuf::__internal::runtime::MiniTablePtr(
                    super::routeguide__Point_msg_init.0,
                )
            })
            .0
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Point {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PointView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <Point as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for PointMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <Point as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Point {
    type Msg = Point;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Point {
    type Msg = Point;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PointMut<'_> {
    type Msg = Point;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PointMut<'_> {
    type Msg = Point;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PointView<'_> {
    type Msg = Point;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Point> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PointMut<'_> {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
impl ::protobuf::OwnedMessageInterop for Point {}
impl<'a> ::protobuf::MessageMutInterop<'a> for PointMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for PointView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.inner.raw().as_ptr() as *const _
    }
}
pub(crate) static mut routeguide__Rectangle_msg_init: ::protobuf::__internal::runtime::MiniTablePtr = ::protobuf::__internal::runtime::MiniTablePtr(
    ::std::ptr::null_mut(),
);
#[allow(non_camel_case_types)]
pub struct Rectangle {
    inner: ::protobuf::__internal::runtime::OwnedMessageInner<Rectangle>,
}
impl ::protobuf::Message for Rectangle {}
impl ::std::default::Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for Rectangle {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
    fn parse_dont_enforce_required(
        serialized: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse_dont_enforce_required(serialized)
    }
}
impl ::std::fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for Rectangle {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
unsafe impl Sync for Rectangle {}
unsafe impl Send for Rectangle {}
impl ::protobuf::Proxied for Rectangle {
    type View<'msg> = RectangleView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for Rectangle {}
impl ::protobuf::MutProxied for Rectangle {
    type Mut<'msg> = RectangleMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RectangleView<'msg> {
    inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Rectangle>,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RectangleView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for RectangleView<'msg> {
    type Message = Rectangle;
}
impl ::std::fmt::Debug for RectangleView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RectangleView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for RectangleView<'_> {
    fn default() -> RectangleView<'static> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(
                ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
            )
        };
        RectangleView::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
impl<'msg> RectangleView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Rectangle>,
    ) -> Self {
        Self {
            inner,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    pub fn to_owned(&self) -> Rectangle {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn has_lo(self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(0) }
    }
    pub fn lo_opt(self) -> ::protobuf::Optional<super::PointView<'msg>> {
        ::protobuf::Optional::new(self.lo(), self.has_lo())
    }
    pub fn lo(self) -> super::PointView<'msg> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(0) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn has_hi(self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(1) }
    }
    pub fn hi_opt(self) -> ::protobuf::Optional<super::PointView<'msg>> {
        ::protobuf::Optional::new(self.hi(), self.has_hi())
    }
    pub fn hi(self) -> super::PointView<'msg> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(1) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
}
unsafe impl Sync for RectangleView<'_> {}
unsafe impl Send for RectangleView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RectangleView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RectangleView<'msg> {}
impl<'msg> ::protobuf::AsView for RectangleView<'msg> {
    type Proxied = Rectangle;
    fn as_view(&self) -> ::protobuf::View<'msg, Rectangle> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RectangleView<'msg> {
    fn into_view<'shorter>(self) -> RectangleView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<Rectangle> for RectangleView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Rectangle {
        let mut dst = Rectangle::new();
        let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(
            &self,
            ::protobuf::__internal::Private,
        );
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst_raw,
                src_raw,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst_arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<Rectangle> for RectangleMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Rectangle {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for Rectangle {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        RectangleView::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn from_message_mut<'msg>(
        msg: ::protobuf::__internal::runtime::RawMessage,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> RectangleMut<'msg> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageMutInner::<
                'msg,
                Rectangle,
            >::wrap_raw(msg, arena)
        };
        RectangleMut::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RectangleMut<'msg> {
    inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Rectangle>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RectangleMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for RectangleMut<'msg> {
    type Message = Rectangle;
}
impl ::std::fmt::Debug for RectangleMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RectangleMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
#[allow(dead_code)]
impl<'msg> RectangleMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent<ParentT: ::protobuf::Message>(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Rectangle>,
    ) -> Self {
        Self { inner }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Rectangle> {
        self.inner
    }
    pub fn to_owned(&self) -> Rectangle {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn has_lo(&self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(0) }
    }
    pub fn clear_lo(&mut self) {
        unsafe {
            self.inner.ptr().clear_field_at_index(0);
        }
    }
    pub fn lo_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.lo(), self.has_lo())
    }
    pub fn lo(&self) -> super::PointView<'_> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(0) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn lo_mut(&mut self) -> super::PointMut<'_> {
        let ptr = unsafe {
            self.inner
                .ptr_mut()
                .get_or_create_mutable_message_at_index(0, self.arena())
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_message_mut_inner(::protobuf::__internal::Private),
            ptr.raw(),
        )
    }
    pub fn set_lo(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut child = val.into_proxied(::protobuf::__internal::Private);
        self.inner
            .arena()
            .fuse(
                ::protobuf::__internal::runtime::UpbGetArena::get_arena(
                    &mut child,
                    ::protobuf::__internal::Private,
                ),
            );
        let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(
            &mut child,
            ::protobuf::__internal::Private,
        );
        unsafe {
            self.inner.ptr_mut().set_base_field_message_at_index(0, child_ptr);
        }
    }
    pub fn has_hi(&self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(1) }
    }
    pub fn clear_hi(&mut self) {
        unsafe {
            self.inner.ptr().clear_field_at_index(1);
        }
    }
    pub fn hi_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.hi(), self.has_hi())
    }
    pub fn hi(&self) -> super::PointView<'_> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(1) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn hi_mut(&mut self) -> super::PointMut<'_> {
        let ptr = unsafe {
            self.inner
                .ptr_mut()
                .get_or_create_mutable_message_at_index(1, self.arena())
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_message_mut_inner(::protobuf::__internal::Private),
            ptr.raw(),
        )
    }
    pub fn set_hi(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut child = val.into_proxied(::protobuf::__internal::Private);
        self.inner
            .arena()
            .fuse(
                ::protobuf::__internal::runtime::UpbGetArena::get_arena(
                    &mut child,
                    ::protobuf::__internal::Private,
                ),
            );
        let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(
            &mut child,
            ::protobuf::__internal::Private,
        );
        unsafe {
            self.inner.ptr_mut().set_base_field_message_at_index(1, child_ptr);
        }
    }
}
unsafe impl Sync for RectangleMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RectangleMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RectangleMut<'msg> {}
impl<'msg> ::protobuf::AsView for RectangleMut<'msg> {
    type Proxied = Rectangle;
    fn as_view(&self) -> ::protobuf::View<'_, Rectangle> {
        RectangleView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RectangleMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Rectangle>
    where
        'msg: 'shorter,
    {
        RectangleView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for RectangleMut<'msg> {
    type MutProxied = Rectangle;
    fn as_mut(&mut self) -> RectangleMut<'msg> {
        RectangleMut { inner: self.inner }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for RectangleMut<'msg> {
    fn into_mut<'shorter>(self) -> RectangleMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl Rectangle {
    pub fn new() -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new(),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Rectangle> {
        ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn parse_dont_enforce_required(
        data: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data)
            .map(|_| msg)
    }
    pub fn as_view(&self) -> RectangleView {
        RectangleView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner),
        )
    }
    pub fn as_mut(&mut self) -> RectangleMut {
        let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(
            &mut self.inner,
        );
        RectangleMut::new(::protobuf::__internal::Private, inner)
    }
    pub fn has_lo(&self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(0) }
    }
    pub fn clear_lo(&mut self) {
        unsafe {
            self.inner.ptr().clear_field_at_index(0);
        }
    }
    pub fn lo_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.lo(), self.has_lo())
    }
    pub fn lo(&self) -> super::PointView<'_> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(0) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn lo_mut(&mut self) -> super::PointMut<'_> {
        let ptr = unsafe {
            self.inner
                .ptr_mut()
                .get_or_create_mutable_message_at_index(0, self.arena())
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_message_mut_inner(::protobuf::__internal::Private),
            ptr.raw(),
        )
    }
    pub fn set_lo(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut child = val.into_proxied(::protobuf::__internal::Private);
        self.inner
            .arena()
            .fuse(
                ::protobuf::__internal::runtime::UpbGetArena::get_arena(
                    &mut child,
                    ::protobuf::__internal::Private,
                ),
            );
        let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(
            &mut child,
            ::protobuf::__internal::Private,
        );
        unsafe {
            self.inner.ptr_mut().set_base_field_message_at_index(0, child_ptr);
        }
    }
    pub fn has_hi(&self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(1) }
    }
    pub fn clear_hi(&mut self) {
        unsafe {
            self.inner.ptr().clear_field_at_index(1);
        }
    }
    pub fn hi_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.hi(), self.has_hi())
    }
    pub fn hi(&self) -> super::PointView<'_> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(1) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn hi_mut(&mut self) -> super::PointMut<'_> {
        let ptr = unsafe {
            self.inner
                .ptr_mut()
                .get_or_create_mutable_message_at_index(1, self.arena())
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_message_mut_inner(::protobuf::__internal::Private),
            ptr.raw(),
        )
    }
    pub fn set_hi(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut child = val.into_proxied(::protobuf::__internal::Private);
        self.inner
            .arena()
            .fuse(
                ::protobuf::__internal::runtime::UpbGetArena::get_arena(
                    &mut child,
                    ::protobuf::__internal::Private,
                ),
            );
        let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(
            &mut child,
            ::protobuf::__internal::Private,
        );
        unsafe {
            self.inner.ptr_mut().set_base_field_message_at_index(1, child_ptr);
        }
    }
}
impl ::std::ops::Drop for Rectangle {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for Rectangle {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for Rectangle {
    type Proxied = Self;
    fn as_view(&self) -> RectangleView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for Rectangle {
    type MutProxied = Self;
    fn as_mut(&mut self) -> RectangleMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Rectangle {
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        static ONCE_LOCK: ::std::sync::OnceLock<
            ::protobuf::__internal::runtime::MiniTablePtr,
        > = ::std::sync::OnceLock::new();
        ONCE_LOCK
            .get_or_init(|| unsafe {
                super::routeguide__Rectangle_msg_init.0 = ::protobuf::__internal::runtime::upb_MiniTable_Build(
                    "$33".as_ptr(),
                    3,
                    ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA
                        .with(|a| a.raw()),
                    ::std::ptr::null_mut(),
                );
                let submessages = [
                    <super::Point as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                    <super::Point as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                ];
                let subenums = [];
                assert!(
                    ::protobuf::__internal::runtime::upb_MiniTable_Link(super::routeguide__Rectangle_msg_init
                    .0, submessages.as_ptr() as * const * const
                    ::protobuf::__internal::runtime::upb_MiniTable, submessages.len(),
                    subenums.as_ptr(), subenums.len())
                );
                ::protobuf::__internal::runtime::MiniTablePtr(
                    super::routeguide__Rectangle_msg_init.0,
                )
            })
            .0
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Rectangle {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RectangleView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <Rectangle as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RectangleMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <Rectangle as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Rectangle {
    type Msg = Rectangle;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Rectangle> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Rectangle {
    type Msg = Rectangle;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Rectangle> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RectangleMut<'_> {
    type Msg = Rectangle;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Rectangle> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RectangleMut<'_> {
    type Msg = Rectangle;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Rectangle> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RectangleView<'_> {
    type Msg = Rectangle;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Rectangle> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RectangleMut<'_> {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
impl ::protobuf::OwnedMessageInterop for Rectangle {}
impl<'a> ::protobuf::MessageMutInterop<'a> for RectangleMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for RectangleView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.inner.raw().as_ptr() as *const _
    }
}
pub(crate) static mut routeguide__Feature_msg_init: ::protobuf::__internal::runtime::MiniTablePtr = ::protobuf::__internal::runtime::MiniTablePtr(
    ::std::ptr::null_mut(),
);
#[allow(non_camel_case_types)]
pub struct Feature {
    inner: ::protobuf::__internal::runtime::OwnedMessageInner<Feature>,
}
impl ::protobuf::Message for Feature {}
impl ::std::default::Default for Feature {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for Feature {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
    fn parse_dont_enforce_required(
        serialized: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse_dont_enforce_required(serialized)
    }
}
impl ::std::fmt::Debug for Feature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for Feature {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
unsafe impl Sync for Feature {}
unsafe impl Send for Feature {}
impl ::protobuf::Proxied for Feature {
    type View<'msg> = FeatureView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for Feature {}
impl ::protobuf::MutProxied for Feature {
    type Mut<'msg> = FeatureMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FeatureView<'msg> {
    inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Feature>,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for FeatureView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for FeatureView<'msg> {
    type Message = Feature;
}
impl ::std::fmt::Debug for FeatureView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for FeatureView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for FeatureView<'_> {
    fn default() -> FeatureView<'static> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(
                ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
            )
        };
        FeatureView::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
impl<'msg> FeatureView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Feature>,
    ) -> Self {
        Self {
            inner,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    pub fn to_owned(&self) -> Feature {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
        let str_view = unsafe { self.inner.ptr().get_string_at_index(0, (b"").into()) };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn has_location(self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(1) }
    }
    pub fn location_opt(self) -> ::protobuf::Optional<super::PointView<'msg>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(self) -> super::PointView<'msg> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(1) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
}
unsafe impl Sync for FeatureView<'_> {}
unsafe impl Send for FeatureView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for FeatureView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for FeatureView<'msg> {}
impl<'msg> ::protobuf::AsView for FeatureView<'msg> {
    type Proxied = Feature;
    fn as_view(&self) -> ::protobuf::View<'msg, Feature> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for FeatureView<'msg> {
    fn into_view<'shorter>(self) -> FeatureView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<Feature> for FeatureView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Feature {
        let mut dst = Feature::new();
        let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(
            &self,
            ::protobuf::__internal::Private,
        );
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst_raw,
                src_raw,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst_arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<Feature> for FeatureMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Feature {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for Feature {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        FeatureView::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn from_message_mut<'msg>(
        msg: ::protobuf::__internal::runtime::RawMessage,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> FeatureMut<'msg> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageMutInner::<
                'msg,
                Feature,
            >::wrap_raw(msg, arena)
        };
        FeatureMut::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FeatureMut<'msg> {
    inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Feature>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for FeatureMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for FeatureMut<'msg> {
    type Message = Feature;
}
impl ::std::fmt::Debug for FeatureMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for FeatureMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
#[allow(dead_code)]
impl<'msg> FeatureMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent<ParentT: ::protobuf::Message>(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Feature>,
    ) -> Self {
        Self { inner }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Feature> {
        self.inner
    }
    pub fn to_owned(&self) -> Feature {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
        let str_view = unsafe { self.inner.ptr().get_string_at_index(0, (b"").into()) };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn set_name(
        &mut self,
        val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>,
    ) {
        let s = val.into_proxied(::protobuf::__internal::Private);
        let (view, arena) = s
            .into_inner(::protobuf::__internal::Private)
            .into_raw_parts();
        let parent_arena = self.inner.arena();
        parent_arena.fuse(&arena);
        unsafe {
            self.inner.ptr_mut().set_base_field_string_at_index(0, view);
        }
    }
    pub fn has_location(&self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(1) }
    }
    pub fn clear_location(&mut self) {
        unsafe {
            self.inner.ptr().clear_field_at_index(1);
        }
    }
    pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(&self) -> super::PointView<'_> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(1) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn location_mut(&mut self) -> super::PointMut<'_> {
        let ptr = unsafe {
            self.inner
                .ptr_mut()
                .get_or_create_mutable_message_at_index(1, self.arena())
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_message_mut_inner(::protobuf::__internal::Private),
            ptr.raw(),
        )
    }
    pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut child = val.into_proxied(::protobuf::__internal::Private);
        self.inner
            .arena()
            .fuse(
                ::protobuf::__internal::runtime::UpbGetArena::get_arena(
                    &mut child,
                    ::protobuf::__internal::Private,
                ),
            );
        let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(
            &mut child,
            ::protobuf::__internal::Private,
        );
        unsafe {
            self.inner.ptr_mut().set_base_field_message_at_index(1, child_ptr);
        }
    }
}
unsafe impl Sync for FeatureMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for FeatureMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for FeatureMut<'msg> {}
impl<'msg> ::protobuf::AsView for FeatureMut<'msg> {
    type Proxied = Feature;
    fn as_view(&self) -> ::protobuf::View<'_, Feature> {
        FeatureView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for FeatureMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Feature>
    where
        'msg: 'shorter,
    {
        FeatureView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for FeatureMut<'msg> {
    type MutProxied = Feature;
    fn as_mut(&mut self) -> FeatureMut<'msg> {
        FeatureMut { inner: self.inner }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for FeatureMut<'msg> {
    fn into_mut<'shorter>(self) -> FeatureMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl Feature {
    pub fn new() -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new(),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Feature> {
        ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn parse_dont_enforce_required(
        data: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data)
            .map(|_| msg)
    }
    pub fn as_view(&self) -> FeatureView {
        FeatureView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner),
        )
    }
    pub fn as_mut(&mut self) -> FeatureMut {
        let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(
            &mut self.inner,
        );
        FeatureMut::new(::protobuf::__internal::Private, inner)
    }
    pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
        let str_view = unsafe { self.inner.ptr().get_string_at_index(0, (b"").into()) };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn set_name(
        &mut self,
        val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>,
    ) {
        let s = val.into_proxied(::protobuf::__internal::Private);
        let (view, arena) = s
            .into_inner(::protobuf::__internal::Private)
            .into_raw_parts();
        let parent_arena = self.inner.arena();
        parent_arena.fuse(&arena);
        unsafe {
            self.inner.ptr_mut().set_base_field_string_at_index(0, view);
        }
    }
    pub fn has_location(&self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(1) }
    }
    pub fn clear_location(&mut self) {
        unsafe {
            self.inner.ptr().clear_field_at_index(1);
        }
    }
    pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(&self) -> super::PointView<'_> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(1) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn location_mut(&mut self) -> super::PointMut<'_> {
        let ptr = unsafe {
            self.inner
                .ptr_mut()
                .get_or_create_mutable_message_at_index(1, self.arena())
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_message_mut_inner(::protobuf::__internal::Private),
            ptr.raw(),
        )
    }
    pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut child = val.into_proxied(::protobuf::__internal::Private);
        self.inner
            .arena()
            .fuse(
                ::protobuf::__internal::runtime::UpbGetArena::get_arena(
                    &mut child,
                    ::protobuf::__internal::Private,
                ),
            );
        let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(
            &mut child,
            ::protobuf::__internal::Private,
        );
        unsafe {
            self.inner.ptr_mut().set_base_field_message_at_index(1, child_ptr);
        }
    }
}
impl ::std::ops::Drop for Feature {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for Feature {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for Feature {
    type Proxied = Self;
    fn as_view(&self) -> FeatureView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for Feature {
    type MutProxied = Self;
    fn as_mut(&mut self) -> FeatureMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Feature {
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        static ONCE_LOCK: ::std::sync::OnceLock<
            ::protobuf::__internal::runtime::MiniTablePtr,
        > = ::std::sync::OnceLock::new();
        ONCE_LOCK
            .get_or_init(|| unsafe {
                super::routeguide__Feature_msg_init.0 = ::protobuf::__internal::runtime::upb_MiniTable_Build(
                    "$1X3".as_ptr(),
                    4,
                    ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA
                        .with(|a| a.raw()),
                    ::std::ptr::null_mut(),
                );
                let submessages = [
                    <super::Point as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                ];
                let subenums = [];
                assert!(
                    ::protobuf::__internal::runtime::upb_MiniTable_Link(super::routeguide__Feature_msg_init
                    .0, submessages.as_ptr() as * const * const
                    ::protobuf::__internal::runtime::upb_MiniTable, submessages.len(),
                    subenums.as_ptr(), subenums.len())
                );
                ::protobuf::__internal::runtime::MiniTablePtr(
                    super::routeguide__Feature_msg_init.0,
                )
            })
            .0
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Feature {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FeatureView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <Feature as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FeatureMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <Feature as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Feature {
    type Msg = Feature;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Feature {
    type Msg = Feature;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FeatureMut<'_> {
    type Msg = Feature;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeatureMut<'_> {
    type Msg = Feature;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeatureView<'_> {
    type Msg = Feature;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<Feature> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FeatureMut<'_> {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
impl ::protobuf::OwnedMessageInterop for Feature {}
impl<'a> ::protobuf::MessageMutInterop<'a> for FeatureMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for FeatureView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.inner.raw().as_ptr() as *const _
    }
}
pub(crate) static mut routeguide__RouteNote_msg_init: ::protobuf::__internal::runtime::MiniTablePtr = ::protobuf::__internal::runtime::MiniTablePtr(
    ::std::ptr::null_mut(),
);
#[allow(non_camel_case_types)]
pub struct RouteNote {
    inner: ::protobuf::__internal::runtime::OwnedMessageInner<RouteNote>,
}
impl ::protobuf::Message for RouteNote {}
impl ::std::default::Default for RouteNote {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for RouteNote {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
    fn parse_dont_enforce_required(
        serialized: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse_dont_enforce_required(serialized)
    }
}
impl ::std::fmt::Debug for RouteNote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteNote {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
unsafe impl Sync for RouteNote {}
unsafe impl Send for RouteNote {}
impl ::protobuf::Proxied for RouteNote {
    type View<'msg> = RouteNoteView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for RouteNote {}
impl ::protobuf::MutProxied for RouteNote {
    type Mut<'msg> = RouteNoteMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RouteNoteView<'msg> {
    inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RouteNote>,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RouteNoteView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for RouteNoteView<'msg> {
    type Message = RouteNote;
}
impl ::std::fmt::Debug for RouteNoteView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteNoteView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for RouteNoteView<'_> {
    fn default() -> RouteNoteView<'static> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(
                ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
            )
        };
        RouteNoteView::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
impl<'msg> RouteNoteView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RouteNote>,
    ) -> Self {
        Self {
            inner,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    pub fn to_owned(&self) -> RouteNote {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn has_location(self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(0) }
    }
    pub fn location_opt(self) -> ::protobuf::Optional<super::PointView<'msg>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(self) -> super::PointView<'msg> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(0) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn message(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
        let str_view = unsafe { self.inner.ptr().get_string_at_index(1, (b"").into()) };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
}
unsafe impl Sync for RouteNoteView<'_> {}
unsafe impl Send for RouteNoteView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RouteNoteView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RouteNoteView<'msg> {}
impl<'msg> ::protobuf::AsView for RouteNoteView<'msg> {
    type Proxied = RouteNote;
    fn as_view(&self) -> ::protobuf::View<'msg, RouteNote> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RouteNoteView<'msg> {
    fn into_view<'shorter>(self) -> RouteNoteView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<RouteNote> for RouteNoteView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteNote {
        let mut dst = RouteNote::new();
        let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(
            &self,
            ::protobuf::__internal::Private,
        );
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst_raw,
                src_raw,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst_arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<RouteNote> for RouteNoteMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteNote {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for RouteNote {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        RouteNoteView::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn from_message_mut<'msg>(
        msg: ::protobuf::__internal::runtime::RawMessage,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> RouteNoteMut<'msg> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageMutInner::<
                'msg,
                RouteNote,
            >::wrap_raw(msg, arena)
        };
        RouteNoteMut::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RouteNoteMut<'msg> {
    inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RouteNote>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RouteNoteMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for RouteNoteMut<'msg> {
    type Message = RouteNote;
}
impl ::std::fmt::Debug for RouteNoteMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteNoteMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
#[allow(dead_code)]
impl<'msg> RouteNoteMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent<ParentT: ::protobuf::Message>(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RouteNote>,
    ) -> Self {
        Self { inner }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RouteNote> {
        self.inner
    }
    pub fn to_owned(&self) -> RouteNote {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn has_location(&self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(0) }
    }
    pub fn clear_location(&mut self) {
        unsafe {
            self.inner.ptr().clear_field_at_index(0);
        }
    }
    pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(&self) -> super::PointView<'_> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(0) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn location_mut(&mut self) -> super::PointMut<'_> {
        let ptr = unsafe {
            self.inner
                .ptr_mut()
                .get_or_create_mutable_message_at_index(0, self.arena())
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_message_mut_inner(::protobuf::__internal::Private),
            ptr.raw(),
        )
    }
    pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut child = val.into_proxied(::protobuf::__internal::Private);
        self.inner
            .arena()
            .fuse(
                ::protobuf::__internal::runtime::UpbGetArena::get_arena(
                    &mut child,
                    ::protobuf::__internal::Private,
                ),
            );
        let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(
            &mut child,
            ::protobuf::__internal::Private,
        );
        unsafe {
            self.inner.ptr_mut().set_base_field_message_at_index(0, child_ptr);
        }
    }
    pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
        let str_view = unsafe { self.inner.ptr().get_string_at_index(1, (b"").into()) };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn set_message(
        &mut self,
        val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>,
    ) {
        let s = val.into_proxied(::protobuf::__internal::Private);
        let (view, arena) = s
            .into_inner(::protobuf::__internal::Private)
            .into_raw_parts();
        let parent_arena = self.inner.arena();
        parent_arena.fuse(&arena);
        unsafe {
            self.inner.ptr_mut().set_base_field_string_at_index(1, view);
        }
    }
}
unsafe impl Sync for RouteNoteMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RouteNoteMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RouteNoteMut<'msg> {}
impl<'msg> ::protobuf::AsView for RouteNoteMut<'msg> {
    type Proxied = RouteNote;
    fn as_view(&self) -> ::protobuf::View<'_, RouteNote> {
        RouteNoteView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RouteNoteMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RouteNote>
    where
        'msg: 'shorter,
    {
        RouteNoteView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for RouteNoteMut<'msg> {
    type MutProxied = RouteNote;
    fn as_mut(&mut self) -> RouteNoteMut<'msg> {
        RouteNoteMut { inner: self.inner }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for RouteNoteMut<'msg> {
    fn into_mut<'shorter>(self) -> RouteNoteMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl RouteNote {
    pub fn new() -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new(),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RouteNote> {
        ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn parse_dont_enforce_required(
        data: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data)
            .map(|_| msg)
    }
    pub fn as_view(&self) -> RouteNoteView {
        RouteNoteView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner),
        )
    }
    pub fn as_mut(&mut self) -> RouteNoteMut {
        let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(
            &mut self.inner,
        );
        RouteNoteMut::new(::protobuf::__internal::Private, inner)
    }
    pub fn has_location(&self) -> bool {
        unsafe { self.inner.ptr().has_field_at_index(0) }
    }
    pub fn clear_location(&mut self) {
        unsafe {
            self.inner.ptr().clear_field_at_index(0);
        }
    }
    pub fn location_opt(&self) -> ::protobuf::Optional<super::PointView<'_>> {
        ::protobuf::Optional::new(self.location(), self.has_location())
    }
    pub fn location(&self) -> super::PointView<'_> {
        let submsg = unsafe { self.inner.ptr().get_message_at_index(0) };
        let raw = submsg
            .map(|ptr| ptr.raw())
            .unwrap_or(::protobuf::__internal::runtime::ScratchSpace::zeroed_block());
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        super::PointView::new(::protobuf::__internal::Private, inner)
    }
    pub fn location_mut(&mut self) -> super::PointMut<'_> {
        let ptr = unsafe {
            self.inner
                .ptr_mut()
                .get_or_create_mutable_message_at_index(0, self.arena())
                .unwrap()
        };
        super::PointMut::from_parent(
            ::protobuf::__internal::Private,
            self.as_message_mut_inner(::protobuf::__internal::Private),
            ptr.raw(),
        )
    }
    pub fn set_location(&mut self, val: impl ::protobuf::IntoProxied<super::Point>) {
        let mut child = val.into_proxied(::protobuf::__internal::Private);
        self.inner
            .arena()
            .fuse(
                ::protobuf::__internal::runtime::UpbGetArena::get_arena(
                    &mut child,
                    ::protobuf::__internal::Private,
                ),
            );
        let child_ptr = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_ptr_mut(
            &mut child,
            ::protobuf::__internal::Private,
        );
        unsafe {
            self.inner.ptr_mut().set_base_field_message_at_index(0, child_ptr);
        }
    }
    pub fn message(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
        let str_view = unsafe { self.inner.ptr().get_string_at_index(1, (b"").into()) };
        unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
    }
    pub fn set_message(
        &mut self,
        val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>,
    ) {
        let s = val.into_proxied(::protobuf::__internal::Private);
        let (view, arena) = s
            .into_inner(::protobuf::__internal::Private)
            .into_raw_parts();
        let parent_arena = self.inner.arena();
        parent_arena.fuse(&arena);
        unsafe {
            self.inner.ptr_mut().set_base_field_string_at_index(1, view);
        }
    }
}
impl ::std::ops::Drop for RouteNote {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for RouteNote {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for RouteNote {
    type Proxied = Self;
    fn as_view(&self) -> RouteNoteView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for RouteNote {
    type MutProxied = Self;
    fn as_mut(&mut self) -> RouteNoteMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RouteNote {
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        static ONCE_LOCK: ::std::sync::OnceLock<
            ::protobuf::__internal::runtime::MiniTablePtr,
        > = ::std::sync::OnceLock::new();
        ONCE_LOCK
            .get_or_init(|| unsafe {
                super::routeguide__RouteNote_msg_init.0 = ::protobuf::__internal::runtime::upb_MiniTable_Build(
                    "$31X".as_ptr(),
                    4,
                    ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA
                        .with(|a| a.raw()),
                    ::std::ptr::null_mut(),
                );
                let submessages = [
                    <super::Point as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                ];
                let subenums = [];
                assert!(
                    ::protobuf::__internal::runtime::upb_MiniTable_Link(super::routeguide__RouteNote_msg_init
                    .0, submessages.as_ptr() as * const * const
                    ::protobuf::__internal::runtime::upb_MiniTable, submessages.len(),
                    subenums.as_ptr(), subenums.len())
                );
                ::protobuf::__internal::runtime::MiniTablePtr(
                    super::routeguide__RouteNote_msg_init.0,
                )
            })
            .0
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RouteNote {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RouteNoteView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <RouteNote as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RouteNoteMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <RouteNote as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RouteNote {
    type Msg = RouteNote;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteNote> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteNote {
    type Msg = RouteNote;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteNote> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RouteNoteMut<'_> {
    type Msg = RouteNote;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteNote> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteNoteMut<'_> {
    type Msg = RouteNote;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteNote> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteNoteView<'_> {
    type Msg = RouteNote;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteNote> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RouteNoteMut<'_> {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
impl ::protobuf::OwnedMessageInterop for RouteNote {}
impl<'a> ::protobuf::MessageMutInterop<'a> for RouteNoteMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for RouteNoteView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.inner.raw().as_ptr() as *const _
    }
}
pub(crate) static mut routeguide__RouteSummary_msg_init: ::protobuf::__internal::runtime::MiniTablePtr = ::protobuf::__internal::runtime::MiniTablePtr(
    ::std::ptr::null_mut(),
);
#[allow(non_camel_case_types)]
pub struct RouteSummary {
    inner: ::protobuf::__internal::runtime::OwnedMessageInner<RouteSummary>,
}
impl ::protobuf::Message for RouteSummary {}
impl ::std::default::Default for RouteSummary {
    fn default() -> Self {
        Self::new()
    }
}
impl ::protobuf::Parse for RouteSummary {
    fn parse(serialized: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse(serialized)
    }
    fn parse_dont_enforce_required(
        serialized: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        Self::parse_dont_enforce_required(serialized)
    }
}
impl ::std::fmt::Debug for RouteSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteSummary {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
unsafe impl Sync for RouteSummary {}
unsafe impl Send for RouteSummary {}
impl ::protobuf::Proxied for RouteSummary {
    type View<'msg> = RouteSummaryView<'msg>;
}
impl ::protobuf::__internal::SealedInternal for RouteSummary {}
impl ::protobuf::MutProxied for RouteSummary {
    type Mut<'msg> = RouteSummaryMut<'msg>;
}
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RouteSummaryView<'msg> {
    inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RouteSummary>,
    _phantom: ::std::marker::PhantomData<&'msg ()>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RouteSummaryView<'msg> {}
impl<'msg> ::protobuf::MessageView<'msg> for RouteSummaryView<'msg> {
    type Message = RouteSummary;
}
impl ::std::fmt::Debug for RouteSummaryView<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteSummaryView<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        let encoded = unsafe {
            ::protobuf::__internal::runtime::wire::encode(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        encoded.map_err(|_| ::protobuf::SerializeError)
    }
}
impl ::std::default::Default for RouteSummaryView<'_> {
    fn default() -> RouteSummaryView<'static> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(
                ::protobuf::__internal::runtime::ScratchSpace::zeroed_block(),
            )
        };
        RouteSummaryView::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
impl<'msg> RouteSummaryView<'msg> {
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RouteSummary>,
    ) -> Self {
        Self {
            inner,
            _phantom: ::std::marker::PhantomData,
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    pub fn to_owned(&self) -> RouteSummary {
        ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
    }
    pub fn point_count(self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(0, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn feature_count(self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(1, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn distance(self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(2, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn elapsed_time(self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(3, (0i32).into()).try_into().unwrap()
        }
    }
}
unsafe impl Sync for RouteSummaryView<'_> {}
unsafe impl Send for RouteSummaryView<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RouteSummaryView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for RouteSummaryView<'msg> {}
impl<'msg> ::protobuf::AsView for RouteSummaryView<'msg> {
    type Proxied = RouteSummary;
    fn as_view(&self) -> ::protobuf::View<'msg, RouteSummary> {
        *self
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RouteSummaryView<'msg> {
    fn into_view<'shorter>(self) -> RouteSummaryView<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
impl<'msg> ::protobuf::IntoProxied<RouteSummary> for RouteSummaryView<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteSummary {
        let mut dst = RouteSummary::new();
        let dst_raw = ::protobuf::__internal::runtime::UpbGetMessagePtrMut::get_raw_message_mut(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let dst_arena = ::protobuf::__internal::runtime::UpbGetArena::get_arena(
            &mut dst,
            ::protobuf::__internal::Private,
        );
        let src_raw = ::protobuf::__internal::runtime::UpbGetMessagePtr::get_raw_message(
            &self,
            ::protobuf::__internal::Private,
        );
        unsafe {
            ::protobuf::__internal::runtime::upb_Message_DeepCopy(
                dst_raw,
                src_raw,
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
                dst_arena.raw(),
            )
        };
        dst
    }
}
impl<'msg> ::protobuf::IntoProxied<RouteSummary> for RouteSummaryMut<'msg> {
    fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RouteSummary {
        ::protobuf::IntoProxied::into_proxied(
            ::protobuf::IntoView::into_view(self),
            _private,
        )
    }
}
impl ::protobuf::__internal::runtime::UpbTypeConversions for RouteSummary {
    fn upb_type() -> ::protobuf::__internal::runtime::CType {
        ::protobuf::__internal::runtime::CType::Message
    }
    fn to_message_value(
        val: ::protobuf::View<'_, Self>,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn into_message_value_fuse_if_required(
        raw_parent_arena: ::protobuf::__internal::runtime::RawArena,
        mut val: Self,
    ) -> ::protobuf::__internal::runtime::upb_MessageValue {
        let parent_arena = ::std::mem::ManuallyDrop::new(unsafe {
            ::protobuf::__internal::runtime::Arena::from_raw(raw_parent_arena)
        });
        parent_arena
            .fuse(val.as_message_mut_inner(::protobuf::__internal::Private).arena());
        ::protobuf::__internal::runtime::upb_MessageValue {
            msg_val: Some(val.raw_msg()),
        }
    }
    unsafe fn from_message_value<'msg>(
        msg: ::protobuf::__internal::runtime::upb_MessageValue,
    ) -> ::protobuf::View<'msg, Self> {
        let raw = unsafe { msg.msg_val }.expect("expected present message value in map");
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        RouteSummaryView::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn from_message_mut<'msg>(
        msg: ::protobuf::__internal::runtime::RawMessage,
        arena: &'msg ::protobuf::__internal::runtime::Arena,
    ) -> RouteSummaryMut<'msg> {
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageMutInner::<
                'msg,
                RouteSummary,
            >::wrap_raw(msg, arena)
        };
        RouteSummaryMut::new(::protobuf::__internal::Private, inner)
    }
}
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RouteSummaryMut<'msg> {
    inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RouteSummary>,
}
impl<'msg> ::protobuf::__internal::SealedInternal for RouteSummaryMut<'msg> {}
impl<'msg> ::protobuf::MessageMut<'msg> for RouteSummaryMut<'msg> {
    type Message = RouteSummary;
}
impl ::std::fmt::Debug for RouteSummaryMut<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let string = unsafe {
            ::protobuf::__internal::runtime::debug_string(
                self.raw_msg(),
                <Self as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            )
        };
        write!(f, "{}", string)
    }
}
impl ::protobuf::Serialize for RouteSummaryMut<'_> {
    fn serialize(&self) -> ::std::result::Result<Vec<u8>, ::protobuf::SerializeError> {
        ::protobuf::AsView::as_view(self).serialize()
    }
}
#[allow(dead_code)]
impl<'msg> RouteSummaryMut<'msg> {
    #[doc(hidden)]
    pub fn from_parent<ParentT: ::protobuf::Message>(
        _private: ::protobuf::__internal::Private,
        parent: ::protobuf::__internal::runtime::MessageMutInner<'msg, ParentT>,
        msg: ::protobuf::__internal::runtime::RawMessage,
    ) -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::MessageMutInner::from_parent(
                parent,
                msg,
            ),
        }
    }
    #[doc(hidden)]
    pub fn new(
        _private: ::protobuf::__internal::Private,
        inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RouteSummary>,
    ) -> Self {
        Self { inner }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RouteSummary> {
        self.inner
    }
    pub fn to_owned(&self) -> RouteSummary {
        ::protobuf::AsView::as_view(self).to_owned()
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn point_count(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(0, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_point_count(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(0, val.into()) }
    }
    pub fn feature_count(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(1, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_feature_count(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(1, val.into()) }
    }
    pub fn distance(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(2, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_distance(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(2, val.into()) }
    }
    pub fn elapsed_time(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(3, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_elapsed_time(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(3, val.into()) }
    }
}
unsafe impl Sync for RouteSummaryMut<'_> {}
impl<'msg> ::protobuf::Proxy<'msg> for RouteSummaryMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for RouteSummaryMut<'msg> {}
impl<'msg> ::protobuf::AsView for RouteSummaryMut<'msg> {
    type Proxied = RouteSummary;
    fn as_view(&self) -> ::protobuf::View<'_, RouteSummary> {
        RouteSummaryView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::IntoView<'msg> for RouteSummaryMut<'msg> {
    fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RouteSummary>
    where
        'msg: 'shorter,
    {
        RouteSummaryView {
            inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(
                self.inner.clone(),
            ),
            _phantom: ::std::marker::PhantomData,
        }
    }
}
impl<'msg> ::protobuf::AsMut for RouteSummaryMut<'msg> {
    type MutProxied = RouteSummary;
    fn as_mut(&mut self) -> RouteSummaryMut<'msg> {
        RouteSummaryMut {
            inner: self.inner,
        }
    }
}
impl<'msg> ::protobuf::IntoMut<'msg> for RouteSummaryMut<'msg> {
    fn into_mut<'shorter>(self) -> RouteSummaryMut<'shorter>
    where
        'msg: 'shorter,
    {
        self
    }
}
#[allow(dead_code)]
impl RouteSummary {
    pub fn new() -> Self {
        Self {
            inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new(),
        }
    }
    fn raw_msg(&self) -> ::protobuf::__internal::runtime::RawMessage {
        self.inner.raw()
    }
    #[doc(hidden)]
    pub fn as_message_mut_inner(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RouteSummary> {
        ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
    }
    fn arena(&mut self) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
    pub fn parse(data: &[u8]) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse(&mut msg, data).map(|_| msg)
    }
    pub fn parse_dont_enforce_required(
        data: &[u8],
    ) -> ::std::result::Result<Self, ::protobuf::ParseError> {
        let mut msg = Self::new();
        ::protobuf::ClearAndParse::clear_and_parse_dont_enforce_required(&mut msg, data)
            .map(|_| msg)
    }
    pub fn as_view(&self) -> RouteSummaryView {
        RouteSummaryView::new(
            ::protobuf::__internal::Private,
            ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner),
        )
    }
    pub fn as_mut(&mut self) -> RouteSummaryMut {
        let inner = ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(
            &mut self.inner,
        );
        RouteSummaryMut::new(::protobuf::__internal::Private, inner)
    }
    pub fn point_count(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(0, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_point_count(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(0, val.into()) }
    }
    pub fn feature_count(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(1, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_feature_count(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(1, val.into()) }
    }
    pub fn distance(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(2, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_distance(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(2, val.into()) }
    }
    pub fn elapsed_time(&self) -> i32 {
        unsafe {
            self.inner.ptr().get_i32_at_index(3, (0i32).into()).try_into().unwrap()
        }
    }
    pub fn set_elapsed_time(&mut self, val: i32) {
        unsafe { self.inner.ptr_mut().set_base_field_i32_at_index(3, val.into()) }
    }
}
impl ::std::ops::Drop for RouteSummary {
    fn drop(&mut self) {}
}
impl ::std::clone::Clone for RouteSummary {
    fn clone(&self) -> Self {
        self.as_view().to_owned()
    }
}
impl ::protobuf::AsView for RouteSummary {
    type Proxied = Self;
    fn as_view(&self) -> RouteSummaryView {
        self.as_view()
    }
}
impl ::protobuf::AsMut for RouteSummary {
    type MutProxied = Self;
    fn as_mut(&mut self) -> RouteSummaryMut {
        self.as_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RouteSummary {
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        static ONCE_LOCK: ::std::sync::OnceLock<
            ::protobuf::__internal::runtime::MiniTablePtr,
        > = ::std::sync::OnceLock::new();
        ONCE_LOCK
            .get_or_init(|| unsafe {
                super::routeguide__RouteSummary_msg_init.0 = ::protobuf::__internal::runtime::upb_MiniTable_Build(
                    "$(P(P(P(P".as_ptr(),
                    9,
                    ::protobuf::__internal::runtime::THREAD_LOCAL_ARENA
                        .with(|a| a.raw()),
                    ::std::ptr::null_mut(),
                );
                let submessages = [];
                let subenums = [];
                assert!(
                    ::protobuf::__internal::runtime::upb_MiniTable_Link(super::routeguide__RouteSummary_msg_init
                    .0, submessages.as_ptr() as * const * const
                    ::protobuf::__internal::runtime::upb_MiniTable, submessages.len(),
                    subenums.as_ptr(), subenums.len())
                );
                ::protobuf::__internal::runtime::MiniTablePtr(
                    super::routeguide__RouteSummary_msg_init.0,
                )
            })
            .0
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RouteSummary {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable
for RouteSummaryView<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <RouteSummary as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable
for RouteSummaryMut<'_> {
    #[inline(always)]
    fn mini_table() -> *const ::protobuf::__internal::runtime::upb_MiniTable {
        <RouteSummary as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RouteSummary {
    type Msg = RouteSummary;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteSummary> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteSummary {
    type Msg = RouteSummary;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteSummary> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut
for RouteSummaryMut<'_> {
    type Msg = RouteSummary;
    fn get_ptr_mut(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteSummary> {
        self.inner.ptr_mut()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteSummaryMut<'_> {
    type Msg = RouteSummary;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteSummary> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RouteSummaryView<'_> {
    type Msg = RouteSummary;
    fn get_ptr(
        &self,
        _private: ::protobuf::__internal::Private,
    ) -> ::protobuf::__internal::runtime::MessagePtr<RouteSummary> {
        self.inner.ptr()
    }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RouteSummaryMut<'_> {
    fn get_arena(
        &mut self,
        _private: ::protobuf::__internal::Private,
    ) -> &::protobuf::__internal::runtime::Arena {
        self.inner.arena()
    }
}
impl ::protobuf::OwnedMessageInterop for RouteSummary {}
impl<'a> ::protobuf::MessageMutInterop<'a> for RouteSummaryMut<'a> {}
impl<'a> ::protobuf::MessageViewInterop<'a> for RouteSummaryView<'a> {
    unsafe fn __unstable_wrap_raw_message(msg: &'a *const ::std::ffi::c_void) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(*msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    unsafe fn __unstable_wrap_raw_message_unchecked_lifetime(
        msg: *const ::std::ffi::c_void,
    ) -> Self {
        let raw = ::protobuf::__internal::runtime::RawMessage::new(msg as *mut _)
            .unwrap();
        let inner = unsafe {
            ::protobuf::__internal::runtime::MessageViewInner::wrap_raw(raw)
        };
        Self::new(::protobuf::__internal::Private, inner)
    }
    fn __unstable_as_raw_message(&self) -> *const ::std::ffi::c_void {
        self.inner.raw().as_ptr() as *const _
    }
}
