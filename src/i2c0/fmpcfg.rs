#[doc = "Register `FMPCFG` reader"]
pub type R = crate::R<FMPCFG_SPEC>;
#[doc = "Register `FMPCFG` writer"]
pub type W = crate::W<FMPCFG_SPEC>;
#[doc = "Field `FMPEN` reader - Fast mode plus enable"]
pub type FMPEN_R = crate::BitReader;
#[doc = "Field `FMPEN` writer - Fast mode plus enable"]
pub type FMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fast mode plus enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FMPEN_R {
        FMPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast mode plus enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpen(&mut self) -> FMPEN_W<FMPCFG_SPEC> {
        FMPEN_W::new(self, 0)
    }
}
#[doc = "Fast mode plus configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmpcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmpcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMPCFG_SPEC;
impl crate::RegisterSpec for FMPCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fmpcfg::R`](R) reader structure"]
impl crate::Readable for FMPCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fmpcfg::W`](W) writer structure"]
impl crate::Writable for FMPCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FMPCFG to value 0"]
impl crate::Resettable for FMPCFG_SPEC {
    const RESET_VALUE: u16 = 0;
}
