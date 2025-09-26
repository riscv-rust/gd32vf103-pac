#[doc = "Register `OVSAMPCTL` reader"]
pub type R = crate::R<OVSAMPCTL_SPEC>;
#[doc = "Register `OVSAMPCTL` writer"]
pub type W = crate::W<OVSAMPCTL_SPEC>;
#[doc = "Field `OVSEN` reader - Oversampler Enable"]
pub type OVSEN_R = crate::BitReader;
#[doc = "Field `OVSEN` writer - Oversampler Enable"]
pub type OVSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSR` reader - Oversampling ratio"]
pub type OVSR_R = crate::FieldReader;
#[doc = "Field `OVSR` writer - Oversampling ratio"]
pub type OVSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVSS` reader - Oversampling shift"]
pub type OVSS_R = crate::FieldReader;
#[doc = "Field `OVSS` writer - Oversampling shift"]
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOVS` reader - Triggered Oversampling"]
pub type TOVS_R = crate::BitReader;
#[doc = "Field `TOVS` writer - Triggered Oversampling"]
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRES` reader - ADC resolution"]
pub type DRES_R = crate::FieldReader;
#[doc = "Field `DRES` writer - ADC resolution"]
pub type DRES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Oversampler Enable"]
    #[inline(always)]
    pub fn ovsen(&self) -> OVSEN_R {
        OVSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - ADC resolution"]
    #[inline(always)]
    pub fn dres(&self) -> DRES_R {
        DRES_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampler Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovsen(&mut self) -> OVSEN_W<OVSAMPCTL_SPEC> {
        OVSEN_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - Oversampling ratio"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<OVSAMPCTL_SPEC> {
        OVSR_W::new(self, 2)
    }
    #[doc = "Bits 5:8 - Oversampling shift"]
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<OVSAMPCTL_SPEC> {
        OVSS_W::new(self, 5)
    }
    #[doc = "Bit 9 - Triggered Oversampling"]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<OVSAMPCTL_SPEC> {
        TOVS_W::new(self, 9)
    }
    #[doc = "Bits 12:13 - ADC resolution"]
    #[inline(always)]
    #[must_use]
    pub fn dres(&mut self) -> DRES_W<OVSAMPCTL_SPEC> {
        DRES_W::new(self, 12)
    }
}
#[doc = "Oversample control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ovsampctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ovsampctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVSAMPCTL_SPEC;
impl crate::RegisterSpec for OVSAMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovsampctl::R`](R) reader structure"]
impl crate::Readable for OVSAMPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ovsampctl::W`](W) writer structure"]
impl crate::Writable for OVSAMPCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OVSAMPCTL to value 0"]
impl crate::Resettable for OVSAMPCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
