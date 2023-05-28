#[doc = "Register `WP` reader"]
pub struct R(crate::R<WP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WP` reader - Store WP\\[31:0\\]
of option bytes block after system reset"]
pub type WP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Store WP\\[31:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(self.bits)
    }
}
#[doc = "Erase/Program Protection register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wp](index.html) module"]
pub struct WP_SPEC;
impl crate::RegisterSpec for WP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wp::R](R) reader structure"]
impl crate::Readable for WP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WP to value 0"]
impl crate::Resettable for WP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
