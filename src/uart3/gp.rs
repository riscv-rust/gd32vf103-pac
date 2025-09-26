#[doc = "Register `GP` reader"]
pub type R = crate::R<GP_SPEC>;
#[doc = "Register `GP` writer"]
pub type W = crate::W<GP_SPEC>;
#[doc = "Field `PSC` reader - Prescaler value"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler value"]
pub type PSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<GP_SPEC, 0> {
        PSC_W::new(self)
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
#[doc = "Guard time and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_SPEC;
impl crate::RegisterSpec for GP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp::R`](R) reader structure"]
impl crate::Readable for GP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp::W`](W) writer structure"]
impl crate::Writable for GP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GP to value 0"]
impl crate::Resettable for GP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
