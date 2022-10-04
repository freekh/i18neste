export default {
  setServerSideI18nesteState: <Ctx, F extends (ctx: Ctx) => Promise<any>>(
    f: F
  ) => {
    console.log("pluginen funker som faen den!");

    return (ctx: Ctx) => f(ctx);
  },
};
