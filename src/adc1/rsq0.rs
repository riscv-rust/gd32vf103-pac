#[doc = "Register `RSQ0` reader"]
pub type R = crate::R<RSQ0_SPEC>;
#[doc = "Register `RSQ0` writer"]
pub type W = crate::W<RSQ0_SPEC>;
#[doc = "Field `RSQ12` reader - 13th conversion in regular sequence"]
pub type RSQ12_R = crate::FieldReader;
#[doc = "Field `RSQ12` writer - 13th conversion in regular sequence"]
pub type RSQ12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ13` reader - 14th conversion in regular sequence"]
pub type RSQ13_R = crate::FieldReader;
#[doc = "Field `RSQ13` writer - 14th conversion in regular sequence"]
pub type RSQ13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ14` reader - 15th conversion in regular sequence"]
pub type RSQ14_R = crate::FieldReader;
#[doc = "Field `RSQ14` writer - 15th conversion in regular sequence"]
pub type RSQ14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ15` reader - 16th conversion in regular sequence"]
pub type RSQ15_R = crate::FieldReader;
#[doc = "Field `RSQ15` writer - 16th conversion in regular sequence"]
pub type RSQ15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RL` reader - Regular channel group length"]
pub type RL_R = crate::FieldReader;
#[doc = "Field `RL` writer - Regular channel group length"]
pub type RL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq12(&self) -> RSQ12_R {
        RSQ12_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq13(&self) -> RSQ13_R {
        RSQ13_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq14(&self) -> RSQ14_R {
        RSQ14_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq15(&self) -> RSQ15_R {
        RSQ15_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - Regular channel group length"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq12(&mut self) -> RSQ12_W<RSQ0_SPEC, 0> {
        RSQ12_W::new(self)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq13(&mut self) -> RSQ13_W<RSQ0_SPEC, 5> {
        RSQ13_W::new(self)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq14(&mut self) -> RSQ14_W<RSQ0_SPEC, 10> {
        RSQ14_W::new(self)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq15(&mut self) -> RSQ15_W<RSQ0_SPEC, 15> {
        RSQ15_W::new(self)
    }
    #[doc = "Bits 20:23 - Regular channel group length"]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RL_W<RSQ0_SPEC, 20> {
        RL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "regular sequence register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsq0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsq0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSQ0_SPEC;
impl crate::RegisterSpec for RSQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsq0::R`](R) reader structure"]
impl crate::Readable for RSQ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsq0::W`](W) writer structure"]
impl crate::Writable for RSQ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSQ0 to value 0"]
impl crate::Resettable for RSQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
