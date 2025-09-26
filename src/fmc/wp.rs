#[doc = "Register `WP` reader"]
pub type R = crate::R<WP_SPEC>;
#[doc = "Field `WP` reader - Store WP\\[31:0\\]
of option bytes block after system reset"]
pub type WP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Store WP\\[31:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(self.bits)
    }
}
#[doc = "Erase/Program Protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WP_SPEC;
impl crate::RegisterSpec for WP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wp::R`](R) reader structure"]
impl crate::Readable for WP_SPEC {}
#[doc = "`reset()` method sets WP to value 0"]
impl crate::Resettable for WP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
