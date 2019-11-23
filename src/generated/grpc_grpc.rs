// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GET_VERSION_GET_VERSION: ::grpcio::Method<super::grpc::GetVersionRequest, super::grpc::GetVersionReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/io.bisq.protobuffer.GetVersion/GetVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GetVersionClient {
    client: ::grpcio::Client,
}

impl GetVersionClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GetVersionClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_version_opt(&self, req: &super::grpc::GetVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::grpc::GetVersionReply> {
        self.client.unary_call(&METHOD_GET_VERSION_GET_VERSION, req, opt)
    }

    pub fn get_version(&self, req: &super::grpc::GetVersionRequest) -> ::grpcio::Result<super::grpc::GetVersionReply> {
        self.get_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_version_async_opt(&self, req: &super::grpc::GetVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetVersionReply>> {
        self.client.unary_call_async(&METHOD_GET_VERSION_GET_VERSION, req, opt)
    }

    pub fn get_version_async(&self, req: &super::grpc::GetVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetVersionReply>> {
        self.get_version_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GetVersion {
    fn get_version(&mut self, ctx: ::grpcio::RpcContext, req: super::grpc::GetVersionRequest, sink: ::grpcio::UnarySink<super::grpc::GetVersionReply>);
}

pub fn create_get_version<S: GetVersion + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GET_VERSION_GET_VERSION, move |ctx, req, resp| {
        instance.get_version(ctx, req, resp)
    });
    builder.build()
}

const METHOD_GET_BALANCE_GET_BALANCE: ::grpcio::Method<super::grpc::GetBalanceRequest, super::grpc::GetBalanceReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/io.bisq.protobuffer.GetBalance/GetBalance",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GetBalanceClient {
    client: ::grpcio::Client,
}

impl GetBalanceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GetBalanceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_balance_opt(&self, req: &super::grpc::GetBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::grpc::GetBalanceReply> {
        self.client.unary_call(&METHOD_GET_BALANCE_GET_BALANCE, req, opt)
    }

    pub fn get_balance(&self, req: &super::grpc::GetBalanceRequest) -> ::grpcio::Result<super::grpc::GetBalanceReply> {
        self.get_balance_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_balance_async_opt(&self, req: &super::grpc::GetBalanceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetBalanceReply>> {
        self.client.unary_call_async(&METHOD_GET_BALANCE_GET_BALANCE, req, opt)
    }

    pub fn get_balance_async(&self, req: &super::grpc::GetBalanceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetBalanceReply>> {
        self.get_balance_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GetBalance {
    fn get_balance(&mut self, ctx: ::grpcio::RpcContext, req: super::grpc::GetBalanceRequest, sink: ::grpcio::UnarySink<super::grpc::GetBalanceReply>);
}

pub fn create_get_balance<S: GetBalance + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GET_BALANCE_GET_BALANCE, move |ctx, req, resp| {
        instance.get_balance(ctx, req, resp)
    });
    builder.build()
}

const METHOD_GET_TRADE_STATISTICS_GET_TRADE_STATISTICS: ::grpcio::Method<super::grpc::GetTradeStatisticsRequest, super::grpc::GetTradeStatisticsReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/io.bisq.protobuffer.GetTradeStatistics/GetTradeStatistics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GetTradeStatisticsClient {
    client: ::grpcio::Client,
}

impl GetTradeStatisticsClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GetTradeStatisticsClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_trade_statistics_opt(&self, req: &super::grpc::GetTradeStatisticsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::grpc::GetTradeStatisticsReply> {
        self.client.unary_call(&METHOD_GET_TRADE_STATISTICS_GET_TRADE_STATISTICS, req, opt)
    }

    pub fn get_trade_statistics(&self, req: &super::grpc::GetTradeStatisticsRequest) -> ::grpcio::Result<super::grpc::GetTradeStatisticsReply> {
        self.get_trade_statistics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_trade_statistics_async_opt(&self, req: &super::grpc::GetTradeStatisticsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetTradeStatisticsReply>> {
        self.client.unary_call_async(&METHOD_GET_TRADE_STATISTICS_GET_TRADE_STATISTICS, req, opt)
    }

    pub fn get_trade_statistics_async(&self, req: &super::grpc::GetTradeStatisticsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetTradeStatisticsReply>> {
        self.get_trade_statistics_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GetTradeStatistics {
    fn get_trade_statistics(&mut self, ctx: ::grpcio::RpcContext, req: super::grpc::GetTradeStatisticsRequest, sink: ::grpcio::UnarySink<super::grpc::GetTradeStatisticsReply>);
}

pub fn create_get_trade_statistics<S: GetTradeStatistics + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GET_TRADE_STATISTICS_GET_TRADE_STATISTICS, move |ctx, req, resp| {
        instance.get_trade_statistics(ctx, req, resp)
    });
    builder.build()
}

const METHOD_GET_OFFERS_GET_OFFERS: ::grpcio::Method<super::grpc::GetOffersRequest, super::grpc::GetOffersReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/io.bisq.protobuffer.GetOffers/GetOffers",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GetOffersClient {
    client: ::grpcio::Client,
}

impl GetOffersClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GetOffersClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_offers_opt(&self, req: &super::grpc::GetOffersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::grpc::GetOffersReply> {
        self.client.unary_call(&METHOD_GET_OFFERS_GET_OFFERS, req, opt)
    }

    pub fn get_offers(&self, req: &super::grpc::GetOffersRequest) -> ::grpcio::Result<super::grpc::GetOffersReply> {
        self.get_offers_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_offers_async_opt(&self, req: &super::grpc::GetOffersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetOffersReply>> {
        self.client.unary_call_async(&METHOD_GET_OFFERS_GET_OFFERS, req, opt)
    }

    pub fn get_offers_async(&self, req: &super::grpc::GetOffersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetOffersReply>> {
        self.get_offers_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GetOffers {
    fn get_offers(&mut self, ctx: ::grpcio::RpcContext, req: super::grpc::GetOffersRequest, sink: ::grpcio::UnarySink<super::grpc::GetOffersReply>);
}

pub fn create_get_offers<S: GetOffers + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GET_OFFERS_GET_OFFERS, move |ctx, req, resp| {
        instance.get_offers(ctx, req, resp)
    });
    builder.build()
}

const METHOD_GET_PAYMENT_ACCOUNTS_GET_PAYMENT_ACCOUNTS: ::grpcio::Method<super::grpc::GetPaymentAccountsRequest, super::grpc::GetPaymentAccountsReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/io.bisq.protobuffer.GetPaymentAccounts/GetPaymentAccounts",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GetPaymentAccountsClient {
    client: ::grpcio::Client,
}

impl GetPaymentAccountsClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GetPaymentAccountsClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_payment_accounts_opt(&self, req: &super::grpc::GetPaymentAccountsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::grpc::GetPaymentAccountsReply> {
        self.client.unary_call(&METHOD_GET_PAYMENT_ACCOUNTS_GET_PAYMENT_ACCOUNTS, req, opt)
    }

    pub fn get_payment_accounts(&self, req: &super::grpc::GetPaymentAccountsRequest) -> ::grpcio::Result<super::grpc::GetPaymentAccountsReply> {
        self.get_payment_accounts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_payment_accounts_async_opt(&self, req: &super::grpc::GetPaymentAccountsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetPaymentAccountsReply>> {
        self.client.unary_call_async(&METHOD_GET_PAYMENT_ACCOUNTS_GET_PAYMENT_ACCOUNTS, req, opt)
    }

    pub fn get_payment_accounts_async(&self, req: &super::grpc::GetPaymentAccountsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::GetPaymentAccountsReply>> {
        self.get_payment_accounts_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait GetPaymentAccounts {
    fn get_payment_accounts(&mut self, ctx: ::grpcio::RpcContext, req: super::grpc::GetPaymentAccountsRequest, sink: ::grpcio::UnarySink<super::grpc::GetPaymentAccountsReply>);
}

pub fn create_get_payment_accounts<S: GetPaymentAccounts + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_GET_PAYMENT_ACCOUNTS_GET_PAYMENT_ACCOUNTS, move |ctx, req, resp| {
        instance.get_payment_accounts(ctx, req, resp)
    });
    builder.build()
}

const METHOD_PLACE_OFFER_PLACE_OFFER: ::grpcio::Method<super::grpc::PlaceOfferRequest, super::grpc::PlaceOfferReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/io.bisq.protobuffer.PlaceOffer/PlaceOffer",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct PlaceOfferClient {
    client: ::grpcio::Client,
}

impl PlaceOfferClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        PlaceOfferClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn place_offer_opt(&self, req: &super::grpc::PlaceOfferRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::grpc::PlaceOfferReply> {
        self.client.unary_call(&METHOD_PLACE_OFFER_PLACE_OFFER, req, opt)
    }

    pub fn place_offer(&self, req: &super::grpc::PlaceOfferRequest) -> ::grpcio::Result<super::grpc::PlaceOfferReply> {
        self.place_offer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn place_offer_async_opt(&self, req: &super::grpc::PlaceOfferRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::PlaceOfferReply>> {
        self.client.unary_call_async(&METHOD_PLACE_OFFER_PLACE_OFFER, req, opt)
    }

    pub fn place_offer_async(&self, req: &super::grpc::PlaceOfferRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::PlaceOfferReply>> {
        self.place_offer_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait PlaceOffer {
    fn place_offer(&mut self, ctx: ::grpcio::RpcContext, req: super::grpc::PlaceOfferRequest, sink: ::grpcio::UnarySink<super::grpc::PlaceOfferReply>);
}

pub fn create_place_offer<S: PlaceOffer + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PLACE_OFFER_PLACE_OFFER, move |ctx, req, resp| {
        instance.place_offer(ctx, req, resp)
    });
    builder.build()
}

const METHOD_STOP_SERVER_STOP_SERVER: ::grpcio::Method<super::grpc::StopServerRequest, super::grpc::StopServerReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/io.bisq.protobuffer.StopServer/StopServer",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct StopServerClient {
    client: ::grpcio::Client,
}

impl StopServerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        StopServerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn stop_server_opt(&self, req: &super::grpc::StopServerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::grpc::StopServerReply> {
        self.client.unary_call(&METHOD_STOP_SERVER_STOP_SERVER, req, opt)
    }

    pub fn stop_server(&self, req: &super::grpc::StopServerRequest) -> ::grpcio::Result<super::grpc::StopServerReply> {
        self.stop_server_opt(req, ::grpcio::CallOption::default())
    }

    pub fn stop_server_async_opt(&self, req: &super::grpc::StopServerRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::StopServerReply>> {
        self.client.unary_call_async(&METHOD_STOP_SERVER_STOP_SERVER, req, opt)
    }

    pub fn stop_server_async(&self, req: &super::grpc::StopServerRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::grpc::StopServerReply>> {
        self.stop_server_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait StopServer {
    fn stop_server(&mut self, ctx: ::grpcio::RpcContext, req: super::grpc::StopServerRequest, sink: ::grpcio::UnarySink<super::grpc::StopServerReply>);
}

pub fn create_stop_server<S: StopServer + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_STOP_SERVER_STOP_SERVER, move |ctx, req, resp| {
        instance.stop_server(ctx, req, resp)
    });
    builder.build()
}
