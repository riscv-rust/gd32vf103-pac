#[doc = "Register `GP` reader"]
pub type R = crate::R<GP_SPEC>;
#[doc = "Register `GP` writer"]
pub type W = crate::W<GP_SPEC>;
#[doc = "Field `PSC` reader - Prescaler value"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - Prescaler value"]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn psc(&mut self) -> PSC_W<GP_SPEC> {
        PSC_W::new(self, 0)
    }
}
#[doc = "Guard time and prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`gp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_SPEC;
impl crate::RegisterSpec for GP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp::R`](R) reader structure"]
impl crate::Readable for GP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp::W`](W) writer structure"]
impl crate::Writable for GP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GP to value 0"]
impl crate::Resettable for GP_SPEC {
    const RESET_VALUE: u32 = 0;
}
