#[doc = "Register `RSQ2` reader"]
pub type R = crate::R<RSQ2_SPEC>;
#[doc = "Register `RSQ2` writer"]
pub type W = crate::W<RSQ2_SPEC>;
#[doc = "Field `RSQ0` reader - 1st conversion in regular sequence"]
pub type RSQ0_R = crate::FieldReader;
#[doc = "Field `RSQ0` writer - 1st conversion in regular sequence"]
pub type RSQ0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ1` reader - 2nd conversion in regular sequence"]
pub type RSQ1_R = crate::FieldReader;
#[doc = "Field `RSQ1` writer - 2nd conversion in regular sequence"]
pub type RSQ1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ2` reader - 3rd conversion in regular sequence"]
pub type RSQ2_R = crate::FieldReader;
#[doc = "Field `RSQ2` writer - 3rd conversion in regular sequence"]
pub type RSQ2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ3` reader - 4th conversion in regular sequence"]
pub type RSQ3_R = crate::FieldReader;
#[doc = "Field `RSQ3` writer - 4th conversion in regular sequence"]
pub type RSQ3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ4` reader - 5th conversion in regular sequence"]
pub type RSQ4_R = crate::FieldReader;
#[doc = "Field `RSQ4` writer - 5th conversion in regular sequence"]
pub type RSQ4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RSQ5` reader - 6th conversion in regular sequence"]
pub type RSQ5_R = crate::FieldReader;
#[doc = "Field `RSQ5` writer - 6th conversion in regular sequence"]
pub type RSQ5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq0(&self) -> RSQ0_R {
        RSQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq1(&self) -> RSQ1_R {
        RSQ1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq2(&self) -> RSQ2_R {
        RSQ2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq3(&self) -> RSQ3_R {
        RSQ3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq4(&self) -> RSQ4_R {
        RSQ4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq5(&self) -> RSQ5_R {
        RSQ5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq0(&mut self) -> RSQ0_W<RSQ2_SPEC, 0> {
        RSQ0_W::new(self)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq1(&mut self) -> RSQ1_W<RSQ2_SPEC, 5> {
        RSQ1_W::new(self)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq2(&mut self) -> RSQ2_W<RSQ2_SPEC, 10> {
        RSQ2_W::new(self)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq3(&mut self) -> RSQ3_W<RSQ2_SPEC, 15> {
        RSQ3_W::new(self)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq4(&mut self) -> RSQ4_W<RSQ2_SPEC, 20> {
        RSQ4_W::new(self)
    }
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    #[must_use]
    pub fn rsq5(&mut self) -> RSQ5_W<RSQ2_SPEC, 25> {
        RSQ5_W::new(self)
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
#[doc = "regular sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsq2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsq2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSQ2_SPEC;
impl crate::RegisterSpec for RSQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsq2::R`](R) reader structure"]
impl crate::Readable for RSQ2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsq2::W`](W) writer structure"]
impl crate::Writable for RSQ2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSQ2 to value 0"]
impl crate::Resettable for RSQ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
