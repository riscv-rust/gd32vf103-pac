#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `CNT` reader - 7-bit counter"]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - 7-bit counter"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WDGTEN` reader - Activation bit"]
pub type WDGTEN_R = crate::BitReader;
#[doc = "Field `WDGTEN` writer - Activation bit"]
pub type WDGTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdgten(&self) -> WDGTEN_R {
        WDGTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CTL_SPEC> {
        CNT_W::new(self, 0)
    }
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    #[must_use]
    pub fn wdgten(&mut self) -> WDGTEN_W<CTL_SPEC> {
        WDGTEN_W::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x7f"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: u32 = 0x7f;
}
