pub mod traits {
    use serde::de::DeserializeOwned;
    use sha1::Digest;

    pub trait LiqPayRequest<Resp, Alg>
    where
        Resp: LiqPayResponse + DeserializeOwned,
        Alg: Digest,
    {
    }

    pub trait LiqPayResponse {}
}

pub mod enums {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub enum Version {
        #[serde(rename = "3")]
        Three,
        #[serde(rename = "7")]
        Seven,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Action {
        #[serde(rename = "pay")]
        Pay,
        #[serde(rename = "invoice_send")]
        SendInvoice,
        #[serde(rename = "invoice_cancel")]
        CancelInvoice,
        #[serde(rename = "payqr")]
        PayQrCode,
        #[serde(rename = "staticQrCreate")]
        CreateQrCode,
        #[serde(rename = "paytoken")]
        PayToken,
        #[serde(rename = "paycash")]
        PayCash,
        #[serde(rename = "paytrack")]
        PayTrack,
        #[serde(rename = "refund")]
        Refund,
        #[serde(rename = "hold")]
        Hold,
        #[serde(rename = "subscribe")]
        Subscribe,
        #[serde(rename = "subscribe_update")]
        UpdateSubscription,
        #[serde(rename = "paydonate")]
        PayDonate,
        #[serde(rename = "auth")]
        Auth,
        #[serde(rename = "status")]
        Status,
        #[serde(rename = "unsubscribe")]
        Unsubscribe,
        #[serde(rename = "ticket")]
        Ticket,
        #[serde(rename = "paysplit")]
        PaySplit,
        #[serde(rename = "regular")]
        Regular,
        #[serde(rename = "payment_prepare")]
        PreparePayment,
        #[serde(rename = "p2pcredit")]
        P2PCredit,
        #[serde(rename = "p2pdebit")]
        P2PDebit,
        #[serde(rename = "p2p")]
        P2P,
        #[serde(rename = "cardverification")]
        CardVerification,
        #[serde(rename = "reports")]
        Reports,
        #[serde(rename = "token_create")]
        CreateToken,
        #[serde(rename = "token_create_unique")]
        CreateUniqueToken,
        #[serde(rename = "token_update")]
        UpdateToken,
        #[serde(rename = "reports_compensation")]
        ReportsCompensation,
        #[serde(rename = "reports_compensation_file")]
        ReportsCompensationFile,
        #[serde(rename = "reports_compensation_file_status")]
        ReportsCompensationFileStatus,
        #[serde(rename = "register")]
        Registry,
        #[serde(rename = "data")]
        Data,
        #[serde(rename = "agent_shop_create")]
        CreateShop,
        #[serde(rename = "agent_shop_register")]
        RegisterShop,
        #[serde(rename = "agent_shop_edit")]
        EditShop,
        #[serde(rename = "agent_info_mcc_codes")]
        MccCodes,
        #[serde(rename = "agent_info_merchant")]
        MerchantInfo,
        #[serde(rename = "agent_info_user")]
        UserInfo,
        #[serde(rename = "invoice_units_get_list")]
        GetInvoiceUnits,
        #[serde(rename = "invoice_units_get_list_by_lang")]
        GetInvoiceUnitsByLanguage,
        #[serde(rename = "confirm")]
        Confirm,
        #[serde(rename = "mpi")]
        Mpi,
    }

    #[derive(Deserialize, Debug)]
    pub enum Bonus {
        #[serde(rename = "bonusplus")]
        BonusPlus,
        #[serde(rename = "discount_club")]
        DiscountClub,
        #[serde(rename = "personal")]
        Personal,
        #[serde(rename = "promo")]
        Promo,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum Currency {
        #[serde(rename = "UAH")]
        UAH,
        #[serde(rename = "EUR")]
        EUR,
        #[serde(rename = "USD")]
        USD,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Language {
        #[serde(rename = "en")]
        En,
        #[serde(rename = "uk")]
        Uk,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum MpiEci {
        #[serde(rename = "5")]
        Success3Ds,
        #[serde(rename = "6")]
        NotSupported3Ds,
        #[serde(rename = "7")]
        Without3Ds,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum PayType {
        #[serde(rename = "card")]
        Card,
        #[serde(rename = "liqpay")]
        LiqPay,
        #[serde(rename = "privat24")]
        Privat24,
        #[serde(rename = "masterpass")]
        Masterpass,
        #[serde(rename = "moment_part")]
        MomentPart,
        #[serde(rename = "paypart")]
        PayPart,
        #[serde(rename = "cash")]
        Cash,
        #[serde(rename = "invoice")]
        Invoice,
        #[serde(rename = "qr")]
        QR,
        #[serde(rename = "apay")]
        ApplePay,
        #[serde(rename = "gpay")]
        GooglePay,
        #[serde(rename = "apay_tavv")]
        ApplePayDecrypted,
        #[serde(rename = "gpay_tavv")]
        GooglePayDecrypted,
        #[serde(rename = "tavv")]
        Tavv,
    }

    #[derive(Deserialize, Debug)]
    pub enum Result {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "error")]
        Error,
    }

    #[derive(Deserialize, Debug)]
    pub enum Status {
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "failure")]
        Failure,
        #[serde(rename = "reversed")]
        Reversed,
        #[serde(rename = "success")]
        Success,
        #[serde(rename = "3ds_verify ")]
        Verify3Ds,
        #[serde(rename = "cvv_verify ")]
        VerifyCvv,
        #[serde(rename = "otp_verify ")]
        VerifyOtp,
        #[serde(rename = "ivr_verify")]
        VerifyIvr,
        #[serde(rename = "password_verify")]
        VerifyPassword,
        #[serde(rename = "phone_verify")]
        VerifyPhone,
        #[serde(rename = "pin_verify")]
        VerifyPin,
        #[serde(rename = "receiver_verify ")]
        VerifyReceiver,
        #[serde(rename = "sender_verify ")]
        VerifySender,
        #[serde(rename = "senderapp_verify ")]
        VerifySenderApp,
        #[serde(rename = "captcha_verify")]
        VerifyCaptcha,
        #[serde(rename = "mp_verify")]
        VerifyMasterPass,
        #[serde(rename = "wait_accept ")]
        WaitAccept,
        #[serde(rename = "wait_card ")]
        WaitCard,
        #[serde(rename = "wait_compensation ")]
        WaitCompensation,
        #[serde(rename = "wait_lc ")]
        WaitLc,
        #[serde(rename = "wait_reserve ")]
        WaitReserve,
        #[serde(rename = "wait_secure ")]
        WaitSecure,
        #[serde(rename = "wait_qr")]
        WaitQr,
        #[serde(rename = "wait_sender")]
        WaitSender,
        #[serde(rename = "cash_wait")]
        WaitCash,
        #[serde(rename = "hold_wait")]
        WaitHold,
        #[serde(rename = "invoice_wait")]
        WaitInvoice,
        #[serde(rename = "subscribed")]
        Subscribed,
        #[serde(rename = "unsubscribed")]
        Unsubscribed,
        #[serde(rename = "prepared")]
        Prepared,
        #[serde(rename = "processing")]
        Processing,
        #[serde(rename = "try_again")]
        TryAgain,
        #[serde(rename = "active")]
        Active,
    }

    #[derive(Debug, Serialize)]
    pub enum Prepare {
        #[serde(rename = "1")]
        Enable,
        #[serde(rename = "tariffs")]
        Tariffs,
    }
}
