#[doc = "Register `RLD` reader"]
pub struct R(crate::R<RLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLD` writer"]
pub struct W(crate::W<RLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLD_SPEC>;
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
impl From<crate::W<RLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RLD` reader - Free watchdog timer counter reload value"]
pub type RLD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RLD` writer - Free watchdog timer counter reload value"]
pub type RLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLD_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Free watchdog timer counter reload value"]
    #[inline(always)]
    pub fn rld(&self) -> RLD_R {
        RLD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Free watchdog timer counter reload value"]
    #[inline(always)]
    #[must_use]
    pub fn rld(&mut self) -> RLD_W<0> {
        RLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rld](index.html) module"]
pub struct RLD_SPEC;
impl crate::RegisterSpec for RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rld::R](R) reader structure"]
impl crate::Readable for RLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rld::W](W) writer structure"]
impl crate::Writable for RLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RLD to value 0x0fff"]
impl crate::Resettable for RLD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
