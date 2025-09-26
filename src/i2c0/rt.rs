#[doc = "Register `RT` reader"]
pub type R = crate::R<RT_SPEC>;
#[doc = "Register `RT` writer"]
pub type W = crate::W<RT_SPEC>;
#[doc = "Field `RISETIME` reader - Maximum rise time in master mode"]
pub type RISETIME_R = crate::FieldReader;
#[doc = "Field `RISETIME` writer - Maximum rise time in master mode"]
pub type RISETIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Maximum rise time in master mode"]
    #[inline(always)]
    pub fn risetime(&self) -> RISETIME_R {
        RISETIME_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Maximum rise time in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn risetime(&mut self) -> RISETIME_W<RT_SPEC, 0> {
        RISETIME_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Rise time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RT_SPEC;
impl crate::RegisterSpec for RT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rt::R`](R) reader structure"]
impl crate::Readable for RT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rt::W`](W) writer structure"]
impl crate::Writable for RT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RT to value 0x02"]
impl crate::Resettable for RT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
