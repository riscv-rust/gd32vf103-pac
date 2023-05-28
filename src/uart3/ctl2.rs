#[doc = "Register `CTL2` reader"]
pub struct R(crate::R<CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL2` writer"]
pub struct W(crate::W<CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL2_SPEC, bool, O>;
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IREN_R = crate::BitReader<bool>;
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL2_SPEC, bool, O>;
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IRLP_R = crate::BitReader<bool>;
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IRLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL2_SPEC, bool, O>;
#[doc = "Field `HDEN` reader - Half-duplex selection"]
pub type HDEN_R = crate::BitReader<bool>;
#[doc = "Field `HDEN` writer - Half-duplex selection"]
pub type HDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL2_SPEC, bool, O>;
#[doc = "Field `DENR` reader - DMA request enable for reception"]
pub type DENR_R = crate::BitReader<bool>;
#[doc = "Field `DENR` writer - DMA request enable for reception"]
pub type DENR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL2_SPEC, bool, O>;
#[doc = "Field `DENT` reader - DMA request enable for transmission"]
pub type DENT_R = crate::BitReader<bool>;
#[doc = "Field `DENT` writer - DMA request enable for transmission"]
pub type DENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA request enable for reception"]
    #[inline(always)]
    pub fn denr(&self) -> DENR_R {
        DENR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA request enable for transmission"]
    #[inline(always)]
    pub fn dent(&self) -> DENT_R {
        DENT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<0> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<1> {
        IREN_W::new(self)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<2> {
        IRLP_W::new(self)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    #[must_use]
    pub fn hden(&mut self) -> HDEN_W<3> {
        HDEN_W::new(self)
    }
    #[doc = "Bit 6 - DMA request enable for reception"]
    #[inline(always)]
    #[must_use]
    pub fn denr(&mut self) -> DENR_W<6> {
        DENR_W::new(self)
    }
    #[doc = "Bit 7 - DMA request enable for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn dent(&mut self) -> DENT_W<7> {
        DENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl2](index.html) module"]
pub struct CTL2_SPEC;
impl crate::RegisterSpec for CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl2::R](R) reader structure"]
impl crate::Readable for CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl2::W](W) writer structure"]
impl crate::Writable for CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
