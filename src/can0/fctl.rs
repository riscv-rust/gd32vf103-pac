#[doc = "Register `FCTL` reader"]
pub type R = crate::R<FCTL_SPEC>;
#[doc = "Register `FCTL` writer"]
pub type W = crate::W<FCTL_SPEC>;
#[doc = "Field `FLD` reader - Filter lock disable"]
pub type FLD_R = crate::BitReader;
#[doc = "Field `FLD` writer - Filter lock disable"]
pub type FLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBC1F` reader - Header bank of CAN1 filter"]
pub type HBC1F_R = crate::FieldReader;
#[doc = "Field `HBC1F` writer - Header bank of CAN1 filter"]
pub type HBC1F_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    pub fn fld(&self) -> FLD_R {
        FLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    pub fn hbc1f(&self) -> HBC1F_R {
        HBC1F_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    #[must_use]
    pub fn fld(&mut self) -> FLD_W<FCTL_SPEC> {
        FLD_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn hbc1f(&mut self) -> HBC1F_W<FCTL_SPEC> {
        HBC1F_W::new(self, 8)
    }
}
#[doc = "Filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTL_SPEC;
impl crate::RegisterSpec for FCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctl::R`](R) reader structure"]
impl crate::Readable for FCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctl::W`](W) writer structure"]
impl crate::Writable for FCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTL to value 0x2a1c_0e01"]
impl crate::Resettable for FCTL_SPEC {
    const RESET_VALUE: u32 = 0x2a1c_0e01;
}
