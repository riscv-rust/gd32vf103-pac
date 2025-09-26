#[doc = "Register `EC` reader"]
pub type R = crate::R<EC_SPEC>;
#[doc = "Register `EC` writer"]
pub type W = crate::W<EC_SPEC>;
#[doc = "Field `PIN` reader - Event output pin selection"]
pub type PIN_R = crate::FieldReader;
#[doc = "Field `PIN` writer - Event output pin selection"]
pub type PIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PORT` reader - Event output port selection"]
pub type PORT_R = crate::FieldReader;
#[doc = "Field `PORT` writer - Event output port selection"]
pub type PORT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EOE` reader - Event output enable"]
pub type EOE_R = crate::BitReader;
#[doc = "Field `EOE` writer - Event output enable"]
pub type EOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Event output pin selection"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Event output port selection"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn eoe(&self) -> EOE_R {
        EOE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Event output pin selection"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PIN_W<EC_SPEC> {
        PIN_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Event output port selection"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PORT_W<EC_SPEC> {
        PORT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoe(&mut self) -> EOE_W<EC_SPEC> {
        EOE_W::new(self, 7)
    }
}
#[doc = "Event control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EC_SPEC;
impl crate::RegisterSpec for EC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ec::R`](R) reader structure"]
impl crate::Readable for EC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ec::W`](W) writer structure"]
impl crate::Writable for EC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EC to value 0"]
impl crate::Resettable for EC_SPEC {
    const RESET_VALUE: u32 = 0;
}
