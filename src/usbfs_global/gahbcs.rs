#[doc = "Register `GAHBCS` reader"]
pub struct R(crate::R<GAHBCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAHBCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAHBCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAHBCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAHBCS` writer"]
pub struct W(crate::W<GAHBCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAHBCS_SPEC>;
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
impl From<crate::W<GAHBCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAHBCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GINTEN` reader - Global interrupt enable"]
pub type GINTEN_R = crate::BitReader<bool>;
#[doc = "Field `GINTEN` writer - Global interrupt enable"]
pub type GINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCS_SPEC, bool, O>;
#[doc = "Field `TXFTH` reader - Tx FIFO threshold"]
pub type TXFTH_R = crate::BitReader<bool>;
#[doc = "Field `TXFTH` writer - Tx FIFO threshold"]
pub type TXFTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCS_SPEC, bool, O>;
#[doc = "Field `PTXFTH` reader - Periodic Tx FIFO threshold"]
pub type PTXFTH_R = crate::BitReader<bool>;
#[doc = "Field `PTXFTH` writer - Periodic Tx FIFO threshold"]
pub type PTXFTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, GAHBCS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn ginten(&self) -> GINTEN_R {
        GINTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Tx FIFO threshold"]
    #[inline(always)]
    pub fn txfth(&self) -> TXFTH_R {
        TXFTH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic Tx FIFO threshold"]
    #[inline(always)]
    pub fn ptxfth(&self) -> PTXFTH_R {
        PTXFTH_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ginten(&mut self) -> GINTEN_W<0> {
        GINTEN_W::new(self)
    }
    #[doc = "Bit 7 - Tx FIFO threshold"]
    #[inline(always)]
    #[must_use]
    pub fn txfth(&mut self) -> TXFTH_W<7> {
        TXFTH_W::new(self)
    }
    #[doc = "Bit 8 - Periodic Tx FIFO threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfth(&mut self) -> PTXFTH_W<8> {
        PTXFTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global AHB control and status register (USBFS_GAHBCS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcs](index.html) module"]
pub struct GAHBCS_SPEC;
impl crate::RegisterSpec for GAHBCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gahbcs::R](R) reader structure"]
impl crate::Readable for GAHBCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gahbcs::W](W) writer structure"]
impl crate::Writable for GAHBCS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAHBCS to value 0"]
impl crate::Resettable for GAHBCS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
