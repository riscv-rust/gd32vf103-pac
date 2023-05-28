#[doc = "Register `CHCTL1_Output` reader"]
pub struct R(crate::R<CHCTL1_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTL1_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTL1_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTL1_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTL1_Output` writer"]
pub struct W(crate::W<CHCTL1_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTL1_OUTPUT_SPEC>;
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
impl From<crate::W<CHCTL1_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTL1_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH2MS` reader - Channel 2 I/O mode selection"]
pub type CH2MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2MS` writer - Channel 2 I/O mode selection"]
pub type CH2MS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CHCTL1_OUTPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH2COMFEN` reader - Channel 2 output compare fast enable"]
pub type CH2COMFEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2COMFEN` writer - Channel 2 output compare fast enable"]
pub type CH2COMFEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `CH2COMSEN` reader - Channel 2 compare output shadow enable"]
pub type CH2COMSEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2COMSEN` writer - Channel 2 compare output shadow enable"]
pub type CH2COMSEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `CH2COMCTL` reader - Channel 2 compare output control"]
pub type CH2COMCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2COMCTL` writer - Channel 2 compare output control"]
pub type CH2COMCTL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CHCTL1_OUTPUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `CH2COMCEN` reader - Channel 2 output compare clear enable"]
pub type CH2COMCEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2COMCEN` writer - Channel 2 output compare clear enable"]
pub type CH2COMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `CH3MS` reader - Channel 3 mode selection"]
pub type CH3MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3MS` writer - Channel 3 mode selection"]
pub type CH3MS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CHCTL1_OUTPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `CH3COMFEN` reader - Channel 3 output compare fast enable"]
pub type CH3COMFEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3COMFEN` writer - Channel 3 output compare fast enable"]
pub type CH3COMFEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `CH3COMSEN` reader - Channel 3 output compare shadow enable"]
pub type CH3COMSEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3COMSEN` writer - Channel 3 output compare shadow enable"]
pub type CH3COMSEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, bool, O>;
#[doc = "Field `CH3COMCTL` reader - Channel 3 compare output control"]
pub type CH3COMCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3COMCTL` writer - Channel 3 compare output control"]
pub type CH3COMCTL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CHCTL1_OUTPUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `CH3COMCEN` reader - Channel 3 output compare clear enable"]
pub type CH3COMCEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3COMCEN` writer - Channel 3 output compare clear enable"]
pub type CH3COMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CHCTL1_OUTPUT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Channel 2 I/O mode selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> CH2MS_R {
        CH2MS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Channel 2 output compare fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&self) -> CH2COMFEN_R {
        CH2COMFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 compare output shadow enable"]
    #[inline(always)]
    pub fn ch2comsen(&self) -> CH2COMSEN_R {
        CH2COMSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 2 compare output control"]
    #[inline(always)]
    pub fn ch2comctl(&self) -> CH2COMCTL_R {
        CH2COMCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&self) -> CH2COMCEN_R {
        CH2COMCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> CH3MS_R {
        CH3MS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel 3 output compare fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&self) -> CH3COMFEN_R {
        CH3COMFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 output compare shadow enable"]
    #[inline(always)]
    pub fn ch3comsen(&self) -> CH3COMSEN_R {
        CH3COMSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Channel 3 compare output control"]
    #[inline(always)]
    pub fn ch3comctl(&self) -> CH3COMCTL_R {
        CH3COMCTL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&self) -> CH3COMCEN_R {
        CH3COMCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 2 I/O mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch2ms(&mut self) -> CH2MS_W<0> {
        CH2MS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 output compare fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2comfen(&mut self) -> CH2COMFEN_W<2> {
        CH2COMFEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2 compare output shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2comsen(&mut self) -> CH2COMSEN_W<3> {
        CH2COMSEN_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 2 compare output control"]
    #[inline(always)]
    #[must_use]
    pub fn ch2comctl(&mut self) -> CH2COMCTL_W<4> {
        CH2COMCTL_W::new(self)
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2comcen(&mut self) -> CH2COMCEN_W<7> {
        CH2COMCEN_W::new(self)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ch3ms(&mut self) -> CH3MS_W<8> {
        CH3MS_W::new(self)
    }
    #[doc = "Bit 10 - Channel 3 output compare fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3comfen(&mut self) -> CH3COMFEN_W<10> {
        CH3COMFEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 output compare shadow enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3comsen(&mut self) -> CH3COMSEN_W<11> {
        CH3COMSEN_W::new(self)
    }
    #[doc = "Bits 12:14 - Channel 3 compare output control"]
    #[inline(always)]
    #[must_use]
    pub fn ch3comctl(&mut self) -> CH3COMCTL_W<12> {
        CH3COMCTL_W::new(self)
    }
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3comcen(&mut self) -> CH3COMCEN_W<15> {
        CH3COMCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel control register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl1_output](index.html) module"]
pub struct CHCTL1_OUTPUT_SPEC;
impl crate::RegisterSpec for CHCTL1_OUTPUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [chctl1_output::R](R) reader structure"]
impl crate::Readable for CHCTL1_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctl1_output::W](W) writer structure"]
impl crate::Writable for CHCTL1_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTL1_Output to value 0"]
impl crate::Resettable for CHCTL1_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
