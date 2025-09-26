#[doc = "Register `CLICINTATTR` reader"]
pub type R = crate::R<CLICINTATTR_SPEC>;
#[doc = "Register `CLICINTATTR` writer"]
pub type W = crate::W<CLICINTATTR_SPEC>;
#[doc = "Field `SHV` reader - SHV"]
pub type SHV_R = crate::BitReader;
#[doc = "Field `SHV` writer - SHV"]
pub type SHV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRIG` reader - TRIG"]
pub type TRIG_R = crate::FieldReader;
#[doc = "Field `TRIG` writer - TRIG"]
pub type TRIG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
    pub fn shv(&mut self) -> SHV_W<CLICINTATTR_SPEC, 0> {
        SHV_W::new(self)
    }
    #[doc = "Bits 1:2 - TRIG"]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TRIG_W<CLICINTATTR_SPEC, 1> {
        TRIG_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "clicintattr Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clicintattr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clicintattr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICINTATTR_SPEC;
impl crate::RegisterSpec for CLICINTATTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintattr::R`](R) reader structure"]
impl crate::Readable for CLICINTATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clicintattr::W`](W) writer structure"]
impl crate::Writable for CLICINTATTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICINTATTR to value 0"]
impl crate::Resettable for CLICINTATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
