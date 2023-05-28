#[doc = "Register `EC` reader"]
pub struct R(crate::R<EC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EC` writer"]
pub struct W(crate::W<EC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EC_SPEC>;
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
impl From<crate::W<EC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN` reader - Event output pin selection"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - Event output pin selection"]
pub type PIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EC_SPEC, u8, u8, 4, O>;
#[doc = "Field `PORT` reader - Event output port selection"]
pub type PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT` writer - Event output port selection"]
pub type PORT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EC_SPEC, u8, u8, 3, O>;
#[doc = "Field `EOE` reader - Event output enable"]
pub type EOE_R = crate::BitReader<bool>;
#[doc = "Field `EOE` writer - Event output enable"]
pub type EOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Event output pin selection"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Event output port selection"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn eoe(&self) -> EOE_R {
        EOE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Event output pin selection"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PIN_W<0> {
        PIN_W::new(self)
    }
    #[doc = "Bits 4:6 - Event output port selection"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PORT_W<4> {
        PORT_W::new(self)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoe(&mut self) -> EOE_W<7> {
        EOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec](index.html) module"]
pub struct EC_SPEC;
impl crate::RegisterSpec for EC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ec::R](R) reader structure"]
impl crate::Readable for EC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ec::W](W) writer structure"]
impl crate::Writable for EC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EC to value 0"]
impl crate::Resettable for EC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
