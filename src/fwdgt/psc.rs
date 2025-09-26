#[doc = "Register `PSC` reader"]
pub type R = crate::R<PSC_SPEC>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PSC_SPEC>;
#[doc = "Field `PSC` reader - Free watchdog timer prescaler selection"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - Free watchdog timer prescaler selection"]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Free watchdog timer prescaler selection"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Free watchdog timer prescaler selection"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<PSC_SPEC> {
        PSC_W::new(self, 0)
    }
}
#[doc = "Prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSC_SPEC;
impl crate::RegisterSpec for PSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PSC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PSC_SPEC {
    const RESET_VALUE: u32 = 0;
}
