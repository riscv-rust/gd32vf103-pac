#[doc = "Register `CAR` reader"]
pub type R = crate::R<CAR_SPEC>;
#[doc = "Register `CAR` writer"]
pub type W = crate::W<CAR_SPEC>;
#[doc = "Field `CARL` reader - Counter auto reload value"]
pub type CARL_R = crate::FieldReader<u16>;
#[doc = "Field `CARL` writer - Counter auto reload value"]
pub type CARL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter auto reload value"]
    #[inline(always)]
    pub fn carl(&self) -> CARL_R {
        CARL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter auto reload value"]
    #[inline(always)]
    #[must_use]
    pub fn carl(&mut self) -> CARL_W<CAR_SPEC> {
        CARL_W::new(self, 0)
    }
}
#[doc = "Counter auto reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`car::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`car::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAR_SPEC;
impl crate::RegisterSpec for CAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`car::R`](R) reader structure"]
impl crate::Readable for CAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`car::W`](W) writer structure"]
impl crate::Writable for CAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CAR to value 0"]
impl crate::Resettable for CAR_SPEC {
    const RESET_VALUE: u16 = 0;
}
