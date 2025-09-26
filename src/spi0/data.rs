#[doc = "Register `DATA` reader"]
pub type R = crate::R<DATA_SPEC>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DATA_SPEC>;
#[doc = "Field `SPI_DATA` reader - Data transfer register"]
pub type SPI_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `SPI_DATA` writer - Data transfer register"]
pub type SPI_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data transfer register"]
    #[inline(always)]
    pub fn spi_data(&self) -> SPI_DATA_R {
        SPI_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data transfer register"]
    #[inline(always)]
    #[must_use]
    pub fn spi_data(&mut self) -> SPI_DATA_W<DATA_SPEC> {
        SPI_DATA_W::new(self, 0)
    }
}
#[doc = "data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u16 = 0;
}
