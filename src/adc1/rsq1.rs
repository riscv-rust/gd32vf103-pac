#[doc = "Register `RSQ1` reader"]
pub type R = crate::R<RSQ1_SPEC>;
#[doc = "Register `RSQ1` writer"]
pub type W = crate::W<RSQ1_SPEC>;
#[doc = "Field `RSQ6` reader - 7th conversion in regular sequence"]
pub type RSQ6_R = crate::FieldReader;
#[doc = "Field `RSQ6` writer - 7th conversion in regular sequence"]
pub type RSQ6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ7` reader - 8th conversion in regular sequence"]
pub type RSQ7_R = crate::FieldReader;
#[doc = "Field `RSQ7` writer - 8th conversion in regular sequence"]
pub type RSQ7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ8` reader - 9th conversion in regular sequence"]
pub type RSQ8_R = crate::FieldReader;
#[doc = "Field `RSQ8` writer - 9th conversion in regular sequence"]
pub type RSQ8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ9` reader - 10th conversion in regular sequence"]
pub type RSQ9_R = crate::FieldReader;
#[doc = "Field `RSQ9` writer - 10th conversion in regular sequence"]
pub type RSQ9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ10` reader - 11th conversion in regular sequence"]
pub type RSQ10_R = crate::FieldReader;
#[doc = "Field `RSQ10` writer - 11th conversion in regular sequence"]
pub type RSQ10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ11` reader - 12th conversion in regular sequence"]
pub type RSQ11_R = crate::FieldReader;
#[doc = "Field `RSQ11` writer - 12th conversion in regular sequence"]
pub type RSQ11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq6(&self) -> RSQ6_R {
        RSQ6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq7(&self) -> RSQ7_R {
        RSQ7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq8(&self) -> RSQ8_R {
        RSQ8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq9(&self) -> RSQ9_R {
        RSQ9_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq10(&self) -> RSQ10_R {
        RSQ10_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq11(&self) -> RSQ11_R {
        RSQ11_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq6(&mut self) -> RSQ6_W<RSQ1_SPEC, 0> {
        RSQ6_W::new(self)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq7(&mut self) -> RSQ7_W<RSQ1_SPEC, 5> {
        RSQ7_W::new(self)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq8(&mut self) -> RSQ8_W<RSQ1_SPEC, 10> {
        RSQ8_W::new(self)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq9(&mut self) -> RSQ9_W<RSQ1_SPEC, 15> {
        RSQ9_W::new(self)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq10(&mut self) -> RSQ10_W<RSQ1_SPEC, 20> {
        RSQ10_W::new(self)
    }
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq11(&mut self) -> RSQ11_W<RSQ1_SPEC, 25> {
        RSQ11_W::new(self)
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
#[doc = "regular sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSQ1_SPEC;
impl crate::RegisterSpec for RSQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsq1::R`](R) reader structure"]
impl crate::Readable for RSQ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsq1::W`](W) writer structure"]
impl crate::Writable for RSQ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSQ1 to value 0"]
impl crate::Resettable for RSQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
