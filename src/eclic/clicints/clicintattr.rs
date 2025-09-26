#[doc = "Register `CLICINTATTR` reader"]
pub type R = crate::R<CLICINTATTR_SPEC>;
#[doc = "Register `CLICINTATTR` writer"]
pub type W = crate::W<CLICINTATTR_SPEC>;
#[doc = "Field `SHV` reader - SHV"]
pub type SHV_R = crate::BitReader;
#[doc = "Field `SHV` writer - SHV"]
pub type SHV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIG` reader - TRIG"]
pub type TRIG_R = crate::FieldReader;
#[doc = "Field `TRIG` writer - TRIG"]
pub type TRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - SHV"]
    #[inline(always)]
    pub fn shv(&self) -> SHV_R {
        SHV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - TRIG"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - SHV"]
    #[inline(always)]
    #[must_use]
    pub fn shv(&mut self) -> SHV_W<CLICINTATTR_SPEC> {
        SHV_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - TRIG"]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TRIG_W<CLICINTATTR_SPEC> {
        TRIG_W::new(self, 1)
    }
}
#[doc = "clicintattr Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicintattr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintattr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICINTATTR_SPEC;
impl crate::RegisterSpec for CLICINTATTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintattr::R`](R) reader structure"]
impl crate::Readable for CLICINTATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clicintattr::W`](W) writer structure"]
impl crate::Writable for CLICINTATTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CLICINTATTR to value 0"]
impl crate::Resettable for CLICINTATTR_SPEC {
    const RESET_VALUE: u8 = 0;
}
