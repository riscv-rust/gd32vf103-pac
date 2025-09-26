#[doc = "Register `IOFF3` reader"]
pub type R = crate::R<IOFF3_SPEC>;
#[doc = "Register `IOFF3` writer"]
pub type W = crate::W<IOFF3_SPEC>;
#[doc = "Field `IOFF` reader - Data offset for inserted channel 3"]
pub type IOFF_R = crate::FieldReader<u16>;
#[doc = "Field `IOFF` writer - Data offset for inserted channel 3"]
pub type IOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for inserted channel 3"]
    #[inline(always)]
    pub fn ioff(&self) -> IOFF_R {
        IOFF_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for inserted channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ioff(&mut self) -> IOFF_W<IOFF3_SPEC> {
        IOFF_W::new(self, 0)
    }
}
#[doc = "Inserted channel data offset register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ioff3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioff3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOFF3_SPEC;
impl crate::RegisterSpec for IOFF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioff3::R`](R) reader structure"]
impl crate::Readable for IOFF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioff3::W`](W) writer structure"]
impl crate::Writable for IOFF3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOFF3 to value 0"]
impl crate::Resettable for IOFF3_SPEC {
    const RESET_VALUE: u32 = 0;
}
