#[doc = "Register `OCTL` reader"]
pub type R = crate::R<OCTL_SPEC>;
#[doc = "Register `OCTL` writer"]
pub type W = crate::W<OCTL_SPEC>;
#[doc = "Field `RCCV` reader - RTC clock calibration value"]
pub type RCCV_R = crate::FieldReader;
#[doc = "Field `RCCV` writer - RTC clock calibration value"]
pub type RCCV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `COEN` reader - RTC clock calibration output enable"]
pub type COEN_R = crate::BitReader;
#[doc = "Field `COEN` writer - RTC clock calibration output enable"]
pub type COEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASOEN` reader - RTC alarm or second signal output enable"]
pub type ASOEN_R = crate::BitReader;
#[doc = "Field `ASOEN` writer - RTC alarm or second signal output enable"]
pub type ASOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROSEL` reader - RTC output selection"]
pub type ROSEL_R = crate::BitReader;
#[doc = "Field `ROSEL` writer - RTC output selection"]
pub type ROSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - RTC clock calibration value"]
    #[inline(always)]
    pub fn rccv(&self) -> RCCV_R {
        RCCV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RTC clock calibration output enable"]
    #[inline(always)]
    pub fn coen(&self) -> COEN_R {
        COEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC alarm or second signal output enable"]
    #[inline(always)]
    pub fn asoen(&self) -> ASOEN_R {
        ASOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC output selection"]
    #[inline(always)]
    pub fn rosel(&self) -> ROSEL_R {
        ROSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - RTC clock calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn rccv(&mut self) -> RCCV_W<OCTL_SPEC, 0> {
        RCCV_W::new(self)
    }
    #[doc = "Bit 7 - RTC clock calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn coen(&mut self) -> COEN_W<OCTL_SPEC, 7> {
        COEN_W::new(self)
    }
    #[doc = "Bit 8 - RTC alarm or second signal output enable"]
    #[inline(always)]
    #[must_use]
    pub fn asoen(&mut self) -> ASOEN_W<OCTL_SPEC, 8> {
        ASOEN_W::new(self)
    }
    #[doc = "Bit 9 - RTC output selection"]
    #[inline(always)]
    #[must_use]
    pub fn rosel(&mut self) -> ROSEL_W<OCTL_SPEC, 9> {
        ROSEL_W::new(self)
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
#[doc = "RTC signal output control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`octl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`octl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCTL_SPEC;
impl crate::RegisterSpec for OCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`octl::R`](R) reader structure"]
impl crate::Readable for OCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`octl::W`](W) writer structure"]
impl crate::Writable for OCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTL to value 0"]
impl crate::Resettable for OCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
