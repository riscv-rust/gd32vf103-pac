#[doc = "Register `HCTL` reader"]
pub type R = crate::R<HCTL_SPEC>;
#[doc = "Register `HCTL` writer"]
pub type W = crate::W<HCTL_SPEC>;
#[doc = "Field `CLKSEL` reader - clock select for USB clock"]
pub type CLKSEL_R = crate::FieldReader;
#[doc = "Field `CLKSEL` writer - clock select for USB clock"]
pub type CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - clock select for USB clock"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - clock select for USB clock"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<HCTL_SPEC> {
        CLKSEL_W::new(self, 0)
    }
}
#[doc = "host configuration register (HCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCTL_SPEC;
impl crate::RegisterSpec for HCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctl::R`](R) reader structure"]
impl crate::Readable for HCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hctl::W`](W) writer structure"]
impl crate::Writable for HCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCTL to value 0"]
impl crate::Resettable for HCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
