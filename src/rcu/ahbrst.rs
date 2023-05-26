#[doc = "Register `AHBRST` reader"]
pub struct R(crate::R<AHBRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBRST` writer"]
pub struct W(crate::W<AHBRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRST_SPEC>;
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
impl From<crate::W<AHBRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBFSRST` reader - USBFS reset"]
pub type USBFSRST_R = crate::BitReader<bool>;
#[doc = "Field `USBFSRST` writer - USBFS reset"]
pub type USBFSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> USBFSRST_R {
        USBFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsrst(&mut self) -> USBFSRST_W<12> {
        USBFSRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrst](index.html) module"]
pub struct AHBRST_SPEC;
impl crate::RegisterSpec for AHBRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbrst::R](R) reader structure"]
impl crate::Readable for AHBRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbrst::W](W) writer structure"]
impl crate::Writable for AHBRST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AHBRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
