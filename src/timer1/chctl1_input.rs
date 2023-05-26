#[doc = "Register `CHCTL1_Input` reader"]
pub struct R(crate::R<CHCTL1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTL1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTL1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTL1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTL1_Input` writer"]
pub struct W(crate::W<CHCTL1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTL1_INPUT_SPEC>;
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
impl From<crate::W<CHCTL1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTL1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH2MS` reader - Channel 2 mode selection"]
pub type CH2MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2MS` writer - Channel 2 mode selection"]
pub type CH2MS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CHCTL1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH2CAPPSC` reader - Channel 2 input capture prescaler"]
pub type CH2CAPPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2CAPPSC` writer - Channel 2 input capture prescaler"]
pub type CH2CAPPSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CHCTL1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH2CAPFLT` reader - Channel 2 input capture filter control"]
pub type CH2CAPFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2CAPFLT` writer - Channel 2 input capture filter control"]
pub type CH2CAPFLT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CHCTL1_INPUT_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH3MS` reader - Channel 3 mode selection"]
pub type CH3MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3MS` writer - Channel 3 mode selection"]
pub type CH3MS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CHCTL1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH3CAPPSC` reader - Channel 3 input capture prescaler"]
pub type CH3CAPPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3CAPPSC` writer - Channel 3 input capture prescaler"]
pub type CH3CAPPSC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CHCTL1_INPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH3CAPFLT` reader - Channel 3 input capture filter control"]
pub type CH3CAPFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3CAPFLT` writer - Channel 3 input capture filter control"]
pub type CH3CAPFLT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CHCTL1_INPUT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> CH2MS_R {
        CH2MS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    pub fn ch2cappsc(&self) -> CH2CAPPSC_R {
        CH2CAPPSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    pub fn ch2capflt(&self) -> CH2CAPFLT_R {
        CH2CAPFLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> CH3MS_R {
        CH3MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    pub fn ch3cappsc(&self) -> CH3CAPPSC_R {
        CH3CAPPSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    pub fn ch3capflt(&self) -> CH3CAPFLT_R {
        CH3CAPFLT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ms(&mut self) -> CH2MS_W<0> {
        CH2MS_W::new(self)
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cappsc(&mut self) -> CH2CAPPSC_W<2> {
        CH2CAPPSC_W::new(self)
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    #[must_use]
    pub fn ch2capflt(&mut self) -> CH2CAPFLT_W<4> {
        CH2CAPFLT_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ms(&mut self) -> CH3MS_W<8> {
        CH3MS_W::new(self)
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cappsc(&mut self) -> CH3CAPPSC_W<10> {
        CH3CAPPSC_W::new(self)
    }
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    #[must_use]
    pub fn ch3capflt(&mut self) -> CH3CAPFLT_W<12> {
        CH3CAPFLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel control register 1 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl1_input](index.html) module"]
pub struct CHCTL1_INPUT_SPEC;
impl crate::RegisterSpec for CHCTL1_INPUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [chctl1_input::R](R) reader structure"]
impl crate::Readable for CHCTL1_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctl1_input::W](W) writer structure"]
impl crate::Writable for CHCTL1_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL1_Input to value 0"]
impl crate::Resettable for CHCTL1_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
