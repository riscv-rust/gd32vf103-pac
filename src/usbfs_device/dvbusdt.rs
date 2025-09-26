#[doc = "Register `DVBUSDT` reader"]
pub type R = crate::R<DVBUSDT_SPEC>;
#[doc = "Register `DVBUSDT` writer"]
pub type W = crate::W<DVBUSDT_SPEC>;
#[doc = "Field `DVBUSDT` reader - Device VBUS discharge time"]
pub type DVBUSDT_R = crate::FieldReader<u16>;
#[doc = "Field `DVBUSDT` writer - Device VBUS discharge time"]
pub type DVBUSDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    pub fn dvbusdt(&self) -> DVBUSDT_R {
        DVBUSDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    #[must_use]
    pub fn dvbusdt(&mut self) -> DVBUSDT_W<DVBUSDT_SPEC, 0> {
        DVBUSDT_W::new(self)
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
#[doc = "device VBUS discharge time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DVBUSDT_SPEC;
impl crate::RegisterSpec for DVBUSDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbusdt::R`](R) reader structure"]
impl crate::Readable for DVBUSDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dvbusdt::W`](W) writer structure"]
impl crate::Writable for DVBUSDT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DVBUSDT to value 0x17d7"]
impl crate::Resettable for DVBUSDT_SPEC {
    const RESET_VALUE: Self::Ux = 0x17d7;
}
