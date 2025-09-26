#[doc = "Register `DATA33` reader"]
pub type R = crate::R<DATA33_SPEC>;
#[doc = "Register `DATA33` writer"]
pub type W = crate::W<DATA33_SPEC>;
#[doc = "Field `DATA` reader - Backup data"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Backup data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DATA33_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "Backup data register 33\n\nYou can [`read`](crate::Reg::read) this register and get [`data33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA33_SPEC;
impl crate::RegisterSpec for DATA33_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`data33::R`](R) reader structure"]
impl crate::Readable for DATA33_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data33::W`](W) writer structure"]
impl crate::Writable for DATA33_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DATA33 to value 0"]
impl crate::Resettable for DATA33_SPEC {
    const RESET_VALUE: u16 = 0;
}
