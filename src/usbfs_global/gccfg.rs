#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GCCFG_SPEC>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GCCFG_SPEC>;
#[doc = "Field `PWRON` reader - Power on"]
pub type PWRON_R = crate::BitReader;
#[doc = "Field `PWRON` writer - Power on"]
pub type PWRON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSACEN` reader - The VBUS A-device Comparer enable"]
pub type VBUSACEN_R = crate::BitReader;
#[doc = "Field `VBUSACEN` writer - The VBUS A-device Comparer enable"]
pub type VBUSACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSBCEN` reader - The VBUS B-device Comparer enable"]
pub type VBUSBCEN_R = crate::BitReader;
#[doc = "Field `VBUSBCEN` writer - The VBUS B-device Comparer enable"]
pub type VBUSBCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFOEN` reader - SOF output enable"]
pub type SOFOEN_R = crate::BitReader;
#[doc = "Field `SOFOEN` writer - SOF output enable"]
pub type SOFOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSIG` reader - VBUS ignored"]
pub type VBUSIG_R = crate::BitReader;
#[doc = "Field `VBUSIG` writer - VBUS ignored"]
pub type VBUSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Power on"]
    #[inline(always)]
    pub fn pwron(&self) -> PWRON_R {
        PWRON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - The VBUS A-device Comparer enable"]
    #[inline(always)]
    pub fn vbusacen(&self) -> VBUSACEN_R {
        VBUSACEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The VBUS B-device Comparer enable"]
    #[inline(always)]
    pub fn vbusbcen(&self) -> VBUSBCEN_R {
        VBUSBCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofoen(&self) -> SOFOEN_R {
        SOFOEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBUS ignored"]
    #[inline(always)]
    pub fn vbusig(&self) -> VBUSIG_R {
        VBUSIG_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Power on"]
    #[inline(always)]
    #[must_use]
    pub fn pwron(&mut self) -> PWRON_W<GCCFG_SPEC> {
        PWRON_W::new(self, 16)
    }
    #[doc = "Bit 18 - The VBUS A-device Comparer enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbusacen(&mut self) -> VBUSACEN_W<GCCFG_SPEC> {
        VBUSACEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - The VBUS B-device Comparer enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbusbcen(&mut self) -> VBUSBCEN_W<GCCFG_SPEC> {
        VBUSBCEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofoen(&mut self) -> SOFOEN_W<GCCFG_SPEC> {
        SOFOEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - VBUS ignored"]
    #[inline(always)]
    #[must_use]
    pub fn vbusig(&mut self) -> VBUSIG_W<GCCFG_SPEC> {
        VBUSIG_W::new(self, 21)
    }
}
#[doc = "Global core configuration register (USBFS_GCCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCCFG_SPEC;
impl crate::RegisterSpec for GCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GCCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GCCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
