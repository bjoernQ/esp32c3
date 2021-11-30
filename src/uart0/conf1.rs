#[doc = "Register `CONF1` reader"]
pub struct R(crate::R<CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF1` writer"]
pub struct W(crate::W<CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_THRHD` reader - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
pub struct RXFIFO_FULL_THRHD_R(crate::FieldReader<u16, u16>);
impl RXFIFO_FULL_THRHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RXFIFO_FULL_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO_FULL_THRHD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO_FULL_THRHD` writer - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
pub struct RXFIFO_FULL_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO_FULL_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
pub struct TXFIFO_EMPTY_THRHD_R(crate::FieldReader<u16, u16>);
impl TXFIFO_EMPTY_THRHD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TXFIFO_EMPTY_THRHD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFO_EMPTY_THRHD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
pub struct TXFIFO_EMPTY_THRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFO_EMPTY_THRHD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 9)) | ((value as u32 & 0x01ff) << 9);
        self.w
    }
}
#[doc = "Field `DIS_RX_DAT_OVF` reader - Disable UART Rx data overflow detect."]
pub struct DIS_RX_DAT_OVF_R(crate::FieldReader<bool, bool>);
impl DIS_RX_DAT_OVF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_RX_DAT_OVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_RX_DAT_OVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_RX_DAT_OVF` writer - Disable UART Rx data overflow detect."]
pub struct DIS_RX_DAT_OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_RX_DAT_OVF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RX_TOUT_FLOW_DIS` reader - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
pub struct RX_TOUT_FLOW_DIS_R(crate::FieldReader<bool, bool>);
impl RX_TOUT_FLOW_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_TOUT_FLOW_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TOUT_FLOW_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TOUT_FLOW_DIS` writer - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
pub struct RX_TOUT_FLOW_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TOUT_FLOW_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `RX_FLOW_EN` reader - This is the flow enable bit for UART receiver."]
pub struct RX_FLOW_EN_R(crate::FieldReader<bool, bool>);
impl RX_FLOW_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FLOW_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FLOW_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FLOW_EN` writer - This is the flow enable bit for UART receiver."]
pub struct RX_FLOW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `RX_TOUT_EN` reader - This is the enble bit for uart receiver's timeout function."]
pub struct RX_TOUT_EN_R(crate::FieldReader<bool, bool>);
impl RX_TOUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_TOUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TOUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TOUT_EN` writer - This is the enble bit for uart receiver's timeout function."]
pub struct RX_TOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TOUT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bit 18 - Disable UART Rx data overflow detect."]
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&self) -> DIS_RX_DAT_OVF_R {
        DIS_RX_DAT_OVF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
    #[inline(always)]
    pub fn rx_tout_flow_dis(&self) -> RX_TOUT_FLOW_DIS_R {
        RX_TOUT_FLOW_DIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - This is the flow enable bit for UART receiver."]
    #[inline(always)]
    pub fn rx_flow_en(&self) -> RX_FLOW_EN_R {
        RX_FLOW_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&self) -> RX_TOUT_EN_R {
        RX_TOUT_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W {
        RXFIFO_FULL_THRHD_W { w: self }
    }
    #[doc = "Bits 9:17 - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W {
        TXFIFO_EMPTY_THRHD_W { w: self }
    }
    #[doc = "Bit 18 - Disable UART Rx data overflow detect."]
    #[inline(always)]
    pub fn dis_rx_dat_ovf(&mut self) -> DIS_RX_DAT_OVF_W {
        DIS_RX_DAT_OVF_W { w: self }
    }
    #[doc = "Bit 19 - Set this bit to stop accumulating idle_cnt when hardware flow control works."]
    #[inline(always)]
    pub fn rx_tout_flow_dis(&mut self) -> RX_TOUT_FLOW_DIS_W {
        RX_TOUT_FLOW_DIS_W { w: self }
    }
    #[doc = "Bit 20 - This is the flow enable bit for UART receiver."]
    #[inline(always)]
    pub fn rx_flow_en(&mut self) -> RX_FLOW_EN_W {
        RX_FLOW_EN_W { w: self }
    }
    #[doc = "Bit 21 - This is the enble bit for uart receiver's timeout function."]
    #[inline(always)]
    pub fn rx_tout_en(&mut self) -> RX_TOUT_EN_W {
        RX_TOUT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1](index.html) module"]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf1::R](R) reader structure"]
impl crate::Readable for CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf1::W](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF1 to value 0xc060"]
impl crate::Resettable for CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc060
    }
}
