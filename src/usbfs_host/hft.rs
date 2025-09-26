#[doc = "Register `HFT` reader"]
pub type R = crate::R<HFT_SPEC>;
#[doc = "Register `HFT` writer"]
pub type W = crate::W<HFT_SPEC>;
#[doc = "Field `FRI` reader - Frame interval"]
pub type FRI_R = crate::FieldReader<u16>;
#[doc = "Field `FRI` writer - Frame interval"]
pub type FRI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn fri(&self) -> FRI_R {
        FRI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    #[must_use]
    pub fn fri(&mut self) -> FRI_W<HFT_SPEC> {
        FRI_W::new(self, 0)
    }
}
#[doc = "Host frame interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`hft::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hft::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFT_SPEC;
impl crate::RegisterSpec for HFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hft::R`](R) reader structure"]
impl crate::Readable for HFT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hft::W`](W) writer structure"]
impl crate::Writable for HFT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFT to value 0xbb80"]
impl crate::Resettable for HFT_SPEC {
    const RESET_VALUE: u32 = 0xbb80;
}
