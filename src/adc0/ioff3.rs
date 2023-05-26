#[doc = "Register `IOFF3` reader"]
pub struct R(crate::R<IOFF3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOFF3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOFF3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOFF3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOFF3` writer"]
pub struct W(crate::W<IOFF3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOFF3_SPEC>;
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
impl From<crate::W<IOFF3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOFF3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IOFF` reader - Data offset for inserted channel 3"]
pub type IOFF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IOFF` writer - Data offset for inserted channel 3"]
pub type IOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IOFF3_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Data offset for inserted channel 3"]
    #[inline(always)]
    pub fn ioff(&self) -> IOFF_R {
        IOFF_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for inserted channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ioff(&mut self) -> IOFF_W<0> {
        IOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inserted channel data offset register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioff3](index.html) module"]
pub struct IOFF3_SPEC;
impl crate::RegisterSpec for IOFF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioff3::R](R) reader structure"]
impl crate::Readable for IOFF3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioff3::W](W) writer structure"]
impl crate::Writable for IOFF3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOFF3 to value 0"]
impl crate::Resettable for IOFF3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
