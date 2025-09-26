#[doc = "Register `STAT0` reader"]
pub type R = crate::R<STAT0_SPEC>;
#[doc = "Register `STAT0` writer"]
pub type W = crate::W<STAT0_SPEC>;
#[doc = "Field `SBSEND` reader - START condition sent out in master mode"]
pub type SBSEND_R = crate::BitReader;
#[doc = "Field `ADDSEND` reader - Address is sent in master mode or received and matches in slave mode"]
pub type ADDSEND_R = crate::BitReader;
#[doc = "Field `BTC` reader - Byte transmission completed"]
pub type BTC_R = crate::BitReader;
#[doc = "Field `ADD10SEND` reader - Header of 10-bit address is sent in master mode"]
pub type ADD10SEND_R = crate::BitReader;
#[doc = "Field `STPDET` reader - STOP condition detected in slave mode"]
pub type STPDET_R = crate::BitReader;
#[doc = "Field `RBNE` reader - I2C_DATA is not Empty during receiving"]
pub type RBNE_R = crate::BitReader;
#[doc = "Field `TBE` reader - I2C_DATA is Empty during transmitting"]
pub type TBE_R = crate::BitReader;
#[doc = "Field `BERR` reader - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `BERR` writer - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
pub type BERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOSTARB` reader - Arbitration Lost in master mode"]
pub type LOSTARB_R = crate::BitReader;
#[doc = "Field `LOSTARB` writer - Arbitration Lost in master mode"]
pub type LOSTARB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AERR` reader - Acknowledge error"]
pub type AERR_R = crate::BitReader;
#[doc = "Field `AERR` writer - Acknowledge error"]
pub type AERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUERR` reader - Over-run or under-run situation occurs in slave mode"]
pub type OUERR_R = crate::BitReader;
#[doc = "Field `OUERR` writer - Over-run or under-run situation occurs in slave mode"]
pub type OUERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PECERR` reader - PEC error when receiving data"]
pub type PECERR_R = crate::BitReader;
#[doc = "Field `PECERR` writer - PEC error when receiving data"]
pub type PECERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBTO` reader - Timeout signal in SMBus mode"]
pub type SMBTO_R = crate::BitReader;
#[doc = "Field `SMBTO` writer - Timeout signal in SMBus mode"]
pub type SMBTO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SMBALT` reader - SMBus Alert status"]
pub type SMBALT_R = crate::BitReader;
#[doc = "Field `SMBALT` writer - SMBus Alert status"]
pub type SMBALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - START condition sent out in master mode"]
    #[inline(always)]
    pub fn sbsend(&self) -> SBSEND_R {
        SBSEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address is sent in master mode or received and matches in slave mode"]
    #[inline(always)]
    pub fn addsend(&self) -> ADDSEND_R {
        ADDSEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transmission completed"]
    #[inline(always)]
    pub fn btc(&self) -> BTC_R {
        BTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Header of 10-bit address is sent in master mode"]
    #[inline(always)]
    pub fn add10send(&self) -> ADD10SEND_R {
        ADD10SEND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STOP condition detected in slave mode"]
    #[inline(always)]
    pub fn stpdet(&self) -> STPDET_R {
        STPDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C_DATA is not Empty during receiving"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C_DATA is Empty during transmitting"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost in master mode"]
    #[inline(always)]
    pub fn lostarb(&self) -> LOSTARB_R {
        LOSTARB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    pub fn aerr(&self) -> AERR_R {
        AERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Over-run or under-run situation occurs in slave mode"]
    #[inline(always)]
    pub fn ouerr(&self) -> OUERR_R {
        OUERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    pub fn smbto(&self) -> SMBTO_R {
        SMBTO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus Alert status"]
    #[inline(always)]
    pub fn smbalt(&self) -> SMBALT_R {
        SMBALT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<STAT0_SPEC, 8> {
        BERR_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration Lost in master mode"]
    #[inline(always)]
    #[must_use]
    pub fn lostarb(&mut self) -> LOSTARB_W<STAT0_SPEC, 9> {
        LOSTARB_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    #[must_use]
    pub fn aerr(&mut self) -> AERR_W<STAT0_SPEC, 10> {
        AERR_W::new(self)
    }
    #[doc = "Bit 11 - Over-run or under-run situation occurs in slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn ouerr(&mut self) -> OUERR_W<STAT0_SPEC, 11> {
        OUERR_W::new(self)
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    #[must_use]
    pub fn pecerr(&mut self) -> PECERR_W<STAT0_SPEC, 12> {
        PECERR_W::new(self)
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    #[must_use]
    pub fn smbto(&mut self) -> SMBTO_W<STAT0_SPEC, 14> {
        SMBTO_W::new(self)
    }
    #[doc = "Bit 15 - SMBus Alert status"]
    #[inline(always)]
    #[must_use]
    pub fn smbalt(&mut self) -> SMBALT_W<STAT0_SPEC, 15> {
        SMBALT_W::new(self)
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
#[doc = "Transfer status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`stat0::R`](R) reader structure"]
impl crate::Readable for STAT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat0::W`](W) writer structure"]
impl crate::Writable for STAT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
